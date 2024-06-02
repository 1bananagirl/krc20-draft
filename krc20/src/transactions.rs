use crate::constants::PROTOCOL_NAMESPACE;
use crate::operations::build_deploy_json_example;

use kaspa_consensus_core::constants::{MAX_TX_IN_SEQUENCE_NUM, TX_VERSION};
use kaspa_consensus_core::subnets::SUBNETWORK_ID_NATIVE;
use kaspa_consensus_core::tx::{
    ScriptPublicKey, Transaction, TransactionInput, TransactionOutpoint, TransactionOutput,
};
use kaspa_txscript::opcodes::codes::*;
use kaspa_txscript::script_builder::{ScriptBuilder, ScriptBuilderResult};
// use std::str::FromStr;
// 

use kaspa_txscript::{
    extract_script_pub_key_address,
    // test_helpers::{create_transaction, op_true_script},
    pay_to_script_hash_script,
    pay_to_script_hash_signature_script,
};



fn build_test_inscription_redeem_envelope() -> ScriptBuilderResult<Vec<u8>> {
    Ok(ScriptBuilder::new()
        .add_op(OpFalse)?
        .add_op(OpIf)?
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

pub fn test(){
    if let Ok(redeem_script) = build_test_inscription_redeem_envelope() {
        
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
        println!(
            "Deploy redeem script envelope in hex: {:02X?}",
            redeem_script
        );
        println!("");
    }
}