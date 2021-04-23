use std::io::Error;
use near_primitives::receipt::{Receipt, ActionReceipt};
use serde::{Deserialize, Serialize};
use near_primitives::transaction::{Action, AddKeyAction, CreateAccountAction, DeleteAccountAction, DeleteKeyAction,
                         DeployContractAction, FunctionCallAction, SignedTransaction, StakeAction, Transaction,
                         TransferAction};
use near_primitives::borsh::{BorshDeserialize, BorshSerialize};
use near_crypto::PublicKey;

#[derive(Serialize, Deserialize)]
struct Person {
    name: String,
    age: u8,
    phones: Vec<String>,
}

fn main() -> Result<(), Error> {
    println!("Start");

    // Read receipts from the dump
    // let data = r#"Receipt { predecessor_id: "wrap.near", receiver_id: "wrap.near", receipt_id: `3RgTMGUKLxZF6BJ6zhwTVgPM3MvykAoSiS9MPKWwEgeR`, receipt: Action(ActionReceipt { signer_id: "67bad843e9ecaab916f0f28ae100b8e0541f4ab3648a0958ac9b3a44eb5031cb", signer_public_key: ed25519:7yvEtACLafq9JCii5SPWh4dercz5BkjTKXck1oSGiWFL, gas_price: 347841850, output_data_receivers: [], input_data_ids: [`G1Dy9q4a7YfyvEQbLZc4b8RDWoYgfwcmU3bxePtRXVJz`], actions: [FunctionCall(FunctionCallAction { method_name: ft_resolve_transfer, args: (146)`{"sender_id":"67bad843e9ecaab916f0f28ae100b8e0541f4ab3648a0958ac9b3a44eb5031cb","receiver_id":"amm.counselor.near","amount":"100…`, gas: 5000000000000, deposit: 0 })] }) }"#.as_bytes();
    let data = r#"Receipt { predecessor_id: "wrap.near", receiver_id: "wrap.near", receipt_id: `3RgTMGUKLxZF6BJ6zhwTVgPM3MvykAoSiS9MPKWwEgeR`, receipt: Action(ActionReceipt { signer_id: "67bad843e9ecaab916f0f28ae100b8e0541f4ab3648a0958ac9b3a44eb5031cb", signer_public_key: ed25519:7yvEtACLafq9JCii5SPWh4dercz5BkjTKXck1oSGiWFL, gas_price: 347841850, output_data_receivers: [], input_data_ids: [`G1Dy9q4a7YfyvEQbLZc4b8RDWoYgfwcmU3bxePtRXVJz`], actions: [] }) }"#.as_bytes();
    let data2 = r#"ed25519:7yvEtACLafq9JCii5SPWh4dercz5BkjTKXck1oSGiWFL"#.as_bytes();

    // let data = r#"
    //
    // ";
    println!("{}", data.len());
    let r: Receipt = Receipt::try_from_slice(data)?; // serde_json::from_str::<Receipt>(data)?;
    let p = PublicKey::try_from_slice(data2)?;
    println!("{:#?}", p);

    // let data2 = r#"
    //     {
    //         "name": "John Doe",
    //         "age": 43,
    //         "phones": [
    //             "+44 1234567",
    //             "+44 2345678"
    //         ]
    //     }"#;
    // let p: Person = serde_json::from_str(data2)?;
    // Do things just like with any other Rust data structure.
    // println!("{:?}", p.phones);

    Ok(())
}
