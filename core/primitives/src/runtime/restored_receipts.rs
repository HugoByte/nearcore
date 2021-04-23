use crate::transaction::{Action, AddKeyAction, CreateAccountAction, DeleteAccountAction, DeleteKeyAction,
                         DeployContractAction, FunctionCallAction, SignedTransaction, StakeAction, Transaction,
                         TransferAction};
use crate::receipt::{Receipt, ActionReceipt};
use near_crypto::{InMemorySigner, KeyType, ED25519PublicKey, PublicKey, Secp256K1PublicKey, SecretKey, Signature};
use crate::hash::CryptoHash;
use std::convert::TryFrom;
use serde_json::json;
use lazy_static::lazy_static;

// pub const RESTORED_RECEIPTS_JSON: Vec<serde_json::Value> = vec![
//     json!({"predecessor_id":"eve.alice.near","receiver_id":"3885505359911f2493f0c40a2bf042981936ec5dddd59708581b155a047864d8","receipt_id":"11111111111111111111111111111111","receipt":{"Action":{"signer_id":"eve.alice.near","signer_public_key":"ed25519:22skMptHjFWNyuEWY22ftn2AbLPSYpmYwGJRGwpNHbTV","gas_price":"1000000000","output_data_receivers":[],"input_data_ids":[],"actions":[{"Transfer":{"deposit":"499999999998639792875000000000000"}}]}}}),
// ];

lazy_static! {
    // RESTORED_RECEIPTS_JSON: Vec<serde_json::Value> = ;

    pub static ref RESTORED_RECEIPTS: Vec<Receipt> = vec![
        serde_json::from_value::<Receipt>(json!({"predecessor_id":"eve.alice.near","receiver_id":"3885505359911f2493f0c40a2bf042981936ec5dddd59708581b155a047864d8","receipt_id":"11111111111111111111111111111111","receipt":{"Action":{"signer_id":"eve.alice.near","signer_public_key":"ed25519:22skMptHjFWNyuEWY22ftn2AbLPSYpmYwGJRGwpNHbTV","gas_price":"1000000000","output_data_receivers":[],"input_data_ids":[],"actions":[{"Transfer":{"deposit":"499999999998639792875000000000000"}}]}}})).unwrap(),
    ];
    // .iter().map(|receipt_json| serde_json::from_value::<Receipt>(receipt_json).unwrap()).collect();
}

// const RESTORED_RECEIPTS: Vec<Receipt> = RESTORED_RECEIPTS_JSON.iter
