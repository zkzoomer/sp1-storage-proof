use alloy::eips::BlockId;
use alloy::network::Ethereum;
use alloy::primitives::{address, b256, Address, B256, U256};
use alloy::providers::{Provider, ProviderBuilder};
use alloy::rpc::client::ClientBuilder;
use alloy::rpc::types::EIP1186AccountProofResponse;
use alloy::transports::layers::RetryBackoffLayer;
use clap::Parser;
use sp1_sdk::{include_elf, ProverClient, SP1Stdin};
use storage_proof_lib::{get_slot, StorageSlotProof};

/// The ELF (executable and linkable format) file for the Succinct RISC-V zkVM.
pub const FIBONACCI_ELF: &[u8] = include_elf!("storage-proof-program");

// Checking ownership of https://cryptopunks.app/cryptopunks/details/5822
// Contract address whose storage is being read
pub const CONTRACT_ADDRESS: Address = address!("b47e3cd837dDF8e4c57f05d70ab865de6e193bbb");  // CryptoPunks
// THe storage slot where the mapping is defined
pub const CONTRACT_MAPPING_SLOT: B256 = b256!("000000000000000000000000000000000000000000000000000000000000000a");  // mapping `punkIndexToAddress`
// Mapping key being read
pub const MAPPING_KEY: B256 = b256!("00000000000000000000000000000000000000000000000000000000000016be");  // Punk #5822
// Expected mapping value
pub const MAPPING_VALUE: Address = address!("c121b520e822d6f69a00a532b1cc9fcbe81b8da0");  // Owner of Punk #5822

// Using an archive block--make sure your RPC gets you an archive node!
pub const BLOCK_NUMBER: u64 = 21000000;  // "Imagine everything, divided by 21 million."
pub const BLOCK_STATE_ROOT: B256 = b256!("9c7216c7718d65de085264ec9d1e564660e445aed3a94e297c95e7b2902f872a");  // https://etherscan.io/block/21000000

/// The arguments for the command.
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(long)]
    execute: bool,
    #[clap(long)]
    prove: bool,
}

#[tokio::main]
async fn main() {
    // Setup the logger.
    sp1_sdk::utils::setup_logger();

    // Parse the command line arguments.
    let args = Args::parse();

    if args.execute == args.prove {
        eprintln!("Error: You must specify either --execute or --prove");
        std::process::exit(1);
    }

    // Setup the client
    dotenv::dotenv().ok();
    let rpc_url: String = std::env::var("ETHEREUM_RPC_URL")
        .expect("ETHEREUM_RPC_URL not set");
    let client = ClientBuilder::default()
        .layer(RetryBackoffLayer::new(100, 50, 300))
        .http(rpc_url.parse()
        .expect("Failed to parse RPC URL"));
    // Setup provider
    #[cfg(target_arch = "wasm32")]
    let client = ClientBuilder::default().http(rpc_url.parse()
        .expect("Failed to parse RPC URL"));
    let provider = ProviderBuilder::new().network::<Ethereum>().on_client(client);

    // Fetch the storage proof
    let slot: B256 = get_slot(&MAPPING_KEY, &CONTRACT_MAPPING_SLOT);
    let account_proof_response: EIP1186AccountProofResponse = provider
        .get_proof(CONTRACT_ADDRESS, vec![slot])
        .block_id(BlockId::from(BLOCK_NUMBER))
        .await
        .unwrap();

    // Setup the prover client.
    let client = ProverClient::new();

    // Setup the inputs.
    let mut stdin = SP1Stdin::new();
    stdin.write::<B256>(&BLOCK_STATE_ROOT);
    stdin.write::<Address>(&CONTRACT_ADDRESS);
    stdin.write::<B256>(&slot);
    stdin.write::<U256>(&U256::from_be_slice(&MAPPING_VALUE.to_vec()));
    stdin.write::<StorageSlotProof>(&StorageSlotProof::from_account_proof_response(account_proof_response));

    if args.execute {
        // Execute the program
        let (_, report) = client.execute(FIBONACCI_ELF, stdin).run().unwrap();
        println!("Program executed successfully.");

        // Record the number of cycles executed.
        println!("Number of cycles: {}", report.total_instruction_count());
    } else {
        // Setup the program for proving.
        let (pk, vk) = client.setup(FIBONACCI_ELF);

        // Generate the proof
        println!("Generating proof...");
        let start_time = std::time::Instant::now();
        let proof = client
            .prove(&pk, stdin)
            .run()
            .expect("failed to generate proof");
        let proving_time = start_time.elapsed();
        println!("Successfully generated proof! Proving time: {:?}", proving_time);

        // Verify the proof.
        client.verify(&proof, &vk).expect("failed to verify proof");
        println!("Successfully verified proof!");
    }
}
