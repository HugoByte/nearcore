use crate::transaction::{Action, AddKeyAction, CreateAccountAction, DeleteAccountAction, DeleteKeyAction,
                         DeployContractAction, FunctionCallAction, SignedTransaction, StakeAction, Transaction,
                         TransferAction};
use crate::receipt::{Receipt, ActionReceipt};
use near_crypto::{InMemorySigner, KeyType, ED25519PublicKey, PublicKey, Secp256K1PublicKey, SecretKey, Signature};
use crate::hash::CryptoHash;
use std::convert::TryFrom;

pub const RESTORED_RECEIPTS: Vec<Receipt> = vec![
    // Receipt {
    //     predecessor_id: String::from("wrap.near"),
    //     receiver_id: String::from("wrap.near"),
    //     receipt_id: CryptoHash::try_from("3RgTMGUKLxZF6BJ6zhwTVgPM3MvykAoSiS9MPKWwEgeR").unwrap(),
    //     receipt: Action(ActionReceipt {
    //         signer_id: String::from("67bad843e9ecaab916f0f28ae100b8e0541f4ab3648a0958ac9b3a44eb5031cb"),
    //         signer_public_key: PublicKey::ED25519(ED25519PublicKey::try_from("7yvEtACLafq9JCii5SPWh4dercz5BkjTKXck1oSGiWFL").unwrap()),
    //         gas_price: 347841850,
    //         output_data_receivers: [],
    //         input_data_ids: ["G1Dy9q4a7YfyvEQbLZc4b8RDWoYgfwcmU3bxePtRXVJz"],
    //         actions: []
    //         // actions: [Action::FunctionCall(FunctionCallAction {
    //         //     method_name: String::from("ft_resolve_transfer"),
    //         //     args: String::from("{\"sender_id\":\"67bad843e9ecaab916f0f28ae100b8e0541f4ab3648a0958ac9b3a44eb5031cb\",\"receiver_id\":\"amm.counselor.near\",\"amount\":\"100\"}"),
    //         //     gas: 5000000000000,
    //         //     deposit: 0 })
    //         // ]
    //     })
    // }
];
