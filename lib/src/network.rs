use serde::{Deserialize, Serialize};
use crate::crypto::PublicKey;
use crate::types::{Block, Transaction, TransactionOutput};
use std::io::{Error as IoError, Read, Write};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum Message {
    FetchUTXOs(PublicKey),
    UTXOs(Vec<(TransactionOutput, bool)>),
    SubmitTransaction(Transaction),
    NewTransaction(Transaction),
    FetchTemplate(Transaction),
    Template(Block),
    ValidateTemplate(Block),
    TemplateValidity(bool),
    SubmitTemplate(Block),
    DiscoverNodes,
    NodeList(Vec<String>),
    AskDifference(u32),
    Difference(i32),
    FetchBlock(usize),
    NewBlock(Block),
}
