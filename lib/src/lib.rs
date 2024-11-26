pub mod proof;

pub use crate::proof::verify_proof;
use alloy::primitives::{keccak256, Bytes, B256, U256};
use alloy::rpc::types::EIP1186AccountProofResponse;
use serde::{Deserialize, Serialize};

/// A storage slot proof
#[derive(Serialize, Deserialize, Debug)]
pub struct StorageSlotProof {
    pub balance: U256,
    pub code_hash: B256,
    pub nonce: u64,
    pub storage_hash: B256,
    pub account_proof: Vec<Bytes>,
    pub storage_proof: Vec<Bytes>,
}

impl StorageSlotProof {
    /// Constructs a new `StorageSlotProof` from an `EIP1186AccountProofResponse` result of an `eth_getProof` call
    pub fn from_account_proof_response(account_proof_response: EIP1186AccountProofResponse) -> Self {
        Self { 
            balance: account_proof_response.balance,
            code_hash: account_proof_response.code_hash,
            nonce: account_proof_response.nonce,
            storage_hash: account_proof_response.storage_hash,
            account_proof: account_proof_response.account_proof,
            storage_proof: account_proof_response.storage_proof[0].proof.clone(),
         }
    }
}

/// Gets the data slot for a given mapping storage slot and key
pub fn get_slot(mapping_key: &B256, mapping_slot: &B256) -> B256 {
    keccak256([&mapping_key.to_vec()[..], &mapping_slot.to_vec()[..]].concat())
}
