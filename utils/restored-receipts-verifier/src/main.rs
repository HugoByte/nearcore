use std::io::Error;
use near_primitives::receipt::{Receipt, ActionReceipt};
use serde::{Deserialize, Serialize};
use serde_json::json;
use near_primitives::transaction::{Action, AddKeyAction, CreateAccountAction, DeleteAccountAction, DeleteKeyAction,
                         DeployContractAction, FunctionCallAction, SignedTransaction, StakeAction, Transaction,
                         TransferAction};
use near_primitives::borsh::{BorshDeserialize, BorshSerialize};
use near_crypto::PublicKey;

use std::fs::File;
use std::io::BufReader;
use std::marker::PhantomData;
use std::path::Path;

fn main() -> Result<(), Error> {
    println!("Start");

    let rx_json = json!({"predecessor_id":"eve.alice.near","receiver_id":"3885505359911f2493f0c40a2bf042981936ec5dddd59708581b155a047864d8","receipt_id":"11111111111111111111111111111111","receipt":{"Action":{"signer_id":"eve.alice.near","signer_public_key":"ed25519:22skMptHjFWNyuEWY22ftn2AbLPSYpmYwGJRGwpNHbTV","gas_price":"1000000000","output_data_receivers":[],"input_data_ids":[],"actions":[{"Transfer":{"deposit":"499999999998639792875000000000000"}}]}}});
    let r = serde_json::from_value::<Receipt>(rx_json)?;
    let rxs = vec![r];
    // println!("{:#?}", r);
    let bytes = rxs.try_to_vec().unwrap();
    let str_path = String::from("./utils/restored-receipts-verifier/receipts.dat");
    let rxs_path = Path::new(&str_path);
    std::fs::write(rxs_path, bytes);

    let bytes = include_bytes!("../receipts.dat");
    let rxs_read = <Vec<Receipt>>::try_from_slice(bytes).unwrap();
    println!("{:#?}", rxs_read);

    Ok(())
}
