use std::io;

use trytes::trits_to_string;
use models::TransactionView;
use models::Transaction;
use serde::ser::{Serialize, Serializer, SerializeSeq, SerializeMap, SerializeStruct};


#[derive(Serialize)]
pub struct TransactionModel {
    //TODO: other fields
    value: i64,
    timestamp: u64,
    current_index: u64,
    last_index: u64,
    bundle: String,
    trunk: String,
    branch: String,
    tag: String,
    attachment_timestamp: u64,
    attachment_timestamp_lower: u64,
    attachment_timestamp_upper: u64,
    nonce: String
}


impl TransactionModel {
    //Convert TransactionView to TransactionModel, so we can serialize into json
    pub fn from_transaction_view(transaction: &Transaction) -> Result<Self, io::Error> {
        let model = TransactionModel{
            value: transaction.value(),
            timestamp: transaction.timestamp(),
            current_index: transaction.current_index(),
            last_index: transaction.last_index(),
            bundle: trits_to_string(&transaction.bundle()).unwrap(),
            trunk: trits_to_string(&transaction.trunk()).unwrap(),
            branch: trits_to_string(&transaction.branch()).unwrap(),
            tag: trits_to_string(&transaction.tag()).unwrap(),
            attachment_timestamp: transaction.attachment_timestamp(),
            attachment_timestamp_lower: transaction.attachment_timestamp_lower(),
            attachment_timestamp_upper: transaction.attachment_timestamp_upper(),
            nonce: trits_to_string(&transaction.nonce()).unwrap(),
        };
        Ok(model)
    }
}
