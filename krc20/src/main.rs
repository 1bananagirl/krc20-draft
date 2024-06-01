use kaspa_consensus_core::constants::{MAX_TX_IN_SEQUENCE_NUM, SOMPI_PER_KASPA, TX_VERSION};
use kaspa_consensus_core::subnets::SUBNETWORK_ID_NATIVE;
use kaspa_consensus_core::tx::{
    ScriptPublicKey, Transaction, TransactionInput, TransactionOutpoint, TransactionOutput,
};
use kaspa_txscript::opcodes::codes::*;
use kaspa_txscript::script_builder::{ScriptBuilder, ScriptBuilderResult};
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use std::str::FromStr;

use kaspa_txscript::{
    extract_script_pub_key_address,
    // test_helpers::{create_transaction, op_true_script},
    pay_to_script_hash_script,
    pay_to_script_hash_signature_script,
};

#[derive(thiserror::Error, Debug)]
pub enum KrcTwentyOpTypeError {
    #[error("Invalid op type: {0}")]
    InvalidOpType(String),
}
pub const FEE_DEPLOY: u64 = 1_000 * SOMPI_PER_KASPA;
pub const FEE_MINT: u64 = SOMPI_PER_KASPA;
pub const PROTOCOL_NAMESPACE: &str = "kasplex";
pub const PROTOCOL_ID: &str = "krc-20";

// Draft of KRC-20 implementation in Rust
// as starting point for structures and functions.
#[derive(Serialize, Deserialize)]
struct DeployData {
    p: String,
    op: String,
    tick: String,
    max: u64,
    lim: u64,
}

fn build_deploy_json_example() -> String {
    let deploy = DeployData {
        p: PROTOCOL_ID.to_string(),
        op: "deploy".to_owned(),
        tick: "test".to_owned(),
        max: 21_000,
        lim: 100,
    };
    serde_json::to_string(&deploy).unwrap()
}

pub enum KrcTwentyOpType {
    Deploy,
    Mint,
    Transfer,
}

impl KrcTwentyOpType {
    pub fn additional_fee(&self) -> u64 {
        match self {
            KrcTwentyOpType::Deploy => FEE_DEPLOY,
            KrcTwentyOpType::Mint => FEE_MINT,
            KrcTwentyOpType::Transfer => 0,
        }
    }
}

impl Display for KrcTwentyOpType {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            KrcTwentyOpType::Mint => "mint",
            KrcTwentyOpType::Deploy => "deploy",
            KrcTwentyOpType::Transfer => "transfer",
        };
        f.write_str(s)
    }
}

impl FromStr for KrcTwentyOpType {
    type Err = KrcTwentyOpTypeError;
    fn from_str(op_type: &str) -> Result<Self, Self::Err> {
        match op_type.to_lowercase().as_str() {
            "deploy" => Ok(KrcTwentyOpType::Deploy),
            "mint" => Ok(KrcTwentyOpType::Mint),
            "transfer" => Ok(KrcTwentyOpType::Transfer),
            _ => Err(KrcTwentyOpTypeError::InvalidOpType(op_type.to_string())),
        }
    }
}

fn build_test_inscription_redeem_envelope() -> ScriptBuilderResult<Vec<u8>> {
    Ok(ScriptBuilder::new()
        .add_op(OpFalse)?
        .add_op(OpIf)?
        .add_op(OpPushData1)?
        .add_data(PROTOCOL_NAMESPACE.as_bytes())?
        .add_data(&[0])?
        .add_data(build_deploy_json_example().as_bytes())?
        .add_op(OpEndIf)?
        .drain())
}

// Commit operation example
pub fn commit_transaction(
    redeem_script: &[u8],
    tx_to_spend: &Transaction,
    fee: u64,
) -> Transaction {
    // OP_BLAKE2B <RedeemScriptHash> OP_EQUAL
    let p2sh = pay_to_script_hash_script(redeem_script);

    // Previous transaction or UTXO to use.
    let previous_outpoint = TransactionOutpoint::new(tx_to_spend.id(), 0);
    let input = TransactionInput::new(
        previous_outpoint,
        tx_to_spend.inputs[0].signature_script.clone(),
        MAX_TX_IN_SEQUENCE_NUM,
        1,
    );
    // tx_to_spend.inputs[0].signature_script.clone() serves here only as example and is not mean to be correct.

    // A transaction is sent to the P2SH address based on the redeem script.
    // The right fee for the Op type must be passed.
    let output = TransactionOutput::new(tx_to_spend.outputs[0].value - fee, p2sh);
    Transaction::new(
        TX_VERSION,
        vec![input],
        vec![output],
        0,
        SUBNETWORK_ID_NATIVE,
        0,
        vec![],
    )

    // Option to add: change address.
}

// Reveal operation example
pub fn reveal_transaction(
    owner_or_receiver: ScriptPublicKey,
    redeem_script: Vec<u8>,
    tx_to_spend: &Transaction,
    fee: u64,
) -> Transaction {
    // The transaction to spend or the UTXO to spend must be the committed transaciton.
    let signature_script = pay_to_script_hash_signature_script(redeem_script, vec![]).unwrap();
    let previous_outpoint = TransactionOutpoint::new(tx_to_spend.id(), 0);

    // Revealing <signature> <RedeemScript>
    let input = TransactionInput::new(
        previous_outpoint,
        signature_script,
        MAX_TX_IN_SEQUENCE_NUM,
        1,
    );

    // Sending the UTXO back to owner (token owner) or receiver (token minting receiver).
    let output = TransactionOutput::new(tx_to_spend.outputs[0].value - fee, owner_or_receiver);
    Transaction::new(
        TX_VERSION,
        vec![input],
        vec![output],
        0,
        SUBNETWORK_ID_NATIVE,
        0,
        vec![],
    )

    // Option to add: change address.
}

fn main() {
    println!("");
    println!("KRC-20 rust draft");
    println!("=======================");
    println!("");
    println!("Mint const: {}", KrcTwentyOpType::Mint);
    println!(
        "Fee for minting: {}",
        KrcTwentyOpType::from_str("mint").unwrap().additional_fee()
    );
    println!(
        "Fee for deploying: {}",
        KrcTwentyOpType::from_str("deploy")
            .unwrap()
            .additional_fee()
    );
    if let Ok(redeem_script) = build_test_inscription_redeem_envelope() {
        println!(
            "Deploy redeem script envelope in hex: {:02X?}",
            redeem_script
        );

        let script_public_key = pay_to_script_hash_script(&redeem_script);
        println!("Redeem script public key: {:?}", script_public_key);

        let extracted =
            extract_script_pub_key_address(&script_public_key, "kaspatest".try_into().unwrap());
        println!("");
        println!("For commit transaction");
        println!("=======================");
        println!("");
        println!(
            "Extracted script pubkey address testnet: {:?}",
            extracted.unwrap()
        );
        println!("");

        println!("For reveal transaction");
        println!("=======================");
        println!("");
        println!("");
    }
}
