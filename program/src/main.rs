// These two lines are necessary for the program to properly compile.
//
// Under the hood, we wrap your main function with some extra code so that it behaves properly
// inside the zkVM.
#![no_main]
sp1_zkvm::entrypoint!(main);

use alloy::primitives::{keccak256, Address, B256, U256};
use alloy::rlp::encode;
use storage_proof_lib::proof::encode_account;
use storage_proof_lib::{verify_proof, StorageSlotProof};

pub fn main() {
    // Read an input to the program.
    //
    // Behind the scenes, this compiles down to a custom system call which handles reading inputs
    // from the prover.
    let state_root: B256 = sp1_zkvm::io::read::<B256>();
    let contract_address: Address = sp1_zkvm::io::read::<Address>();
    let slot: B256 = sp1_zkvm::io::read::<B256>();
    let value: U256 = sp1_zkvm::io::read::<U256>();
    let slot_proof: StorageSlotProof = sp1_zkvm::io::read::<StorageSlotProof>();

    let account_path: Vec<u8> = keccak256(contract_address).to_vec();
    let account_encoded: Vec<u8> = encode_account(&slot_proof);

    // Validate account state
    assert!(
        verify_proof(
            &slot_proof.account_proof,
            state_root.as_slice(),
            &account_path,
            &account_encoded,
        )
    );

    // Validate storage slot value
    let slot_hash: B256 = keccak256(slot);
    assert!(
        verify_proof(
            &slot_proof.storage_proof,
            slot_proof.storage_hash.as_slice(),
            slot_hash.as_slice(),
            &encode(value),
        )
    );
}
