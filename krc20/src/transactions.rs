// use crate::constants::KASPLEX_HEADER;
use crate::constants::PROTOCOL_NAMESPACE;

use crate::operations::{
    build_deploy_json_example, build_mint_json_example, build_transfer_json_example,
};

use crate::detect::detect_krc20;

use kaspa_addresses::{Address, Prefix, Version};
use kaspa_consensus_core::constants::{MAX_TX_IN_SEQUENCE_NUM, TX_VERSION};
// use kaspa_consensus_core::hashing::sighash::SigHashReusedValues;
use kaspa_consensus_core::subnets::SUBNETWORK_ID_NATIVE;
use kaspa_consensus_core::tx::{
    PopulatedTransaction, ScriptPublicKey, Transaction, TransactionId, TransactionInput,
    TransactionOutpoint, TransactionOutput, UtxoEntry, VerifiableTransaction,
};
// use kaspa_txscript::TxScriptEngine;
//PopulatedTransaction,
// use kaspa_txscript::get_sig_op_count;
use kaspa_txscript::opcodes::codes::*;
use kaspa_txscript::script_builder::{ScriptBuilder, ScriptBuilderResult};
use kaspa_txscript::script_class::ScriptClass;
use workflow_log::log_info;
// use std::str::FromStr;
//
use std::ascii::escape_default;
use std::str;

// use itertools::Itertools;
// use kaspa_txscript::opcodes::{deserialize_next_opcode, OpCodeImplementation};
// use kaspa_txscript_errors::TxScriptError;
// use std::iter::once;

use kaspa_txscript::{
    extract_script_pub_key_address,
    // test_helpers::{create_transaction, op_true_script},
    pay_to_script_hash_script,
    pay_to_script_hash_signature_script,
};

fn build_test_inscription_redeem_script(
    redeem_script: &[u8],
    address: Address,
) -> ScriptBuilderResult<Vec<u8>> {
    Ok(ScriptBuilder::new()
        .add_data(address.payload.as_slice())?
        .add_op(OpCheckSig)?
        .add_op(OpFalse)?
        .add_op(OpIf)?
        .add_data(PROTOCOL_NAMESPACE.as_bytes())?
        .add_data(&[0])?
        .add_data(redeem_script)?
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
    let input = TransactionInput::new(previous_outpoint, vec![], MAX_TX_IN_SEQUENCE_NUM, 1);

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

fn bytes_as_string_format(bs: &[u8]) -> String {
    let mut visible = String::new();
    for &b in bs {
        let part: Vec<u8> = escape_default(b).collect();
        visible.push_str(str::from_utf8(&part).unwrap());
    }
    visible
}

fn create_spending_transaction(
    sig_script: Vec<u8>,
    script_public_key: ScriptPublicKey,
) -> Transaction {
    let coinbase = Transaction::new(
        1,
        vec![TransactionInput::new(
            TransactionOutpoint::new(TransactionId::default(), 0xffffffffu32),
            vec![0, 0],
            MAX_TX_IN_SEQUENCE_NUM,
            Default::default(),
        )],
        vec![TransactionOutput::new(0, script_public_key)],
        Default::default(),
        Default::default(),
        Default::default(),
        Default::default(),
    );

    Transaction::new(
        1,
        vec![TransactionInput::new(
            TransactionOutpoint::new(coinbase.id(), 0u32),
            sig_script,
            MAX_TX_IN_SEQUENCE_NUM,
            Default::default(),
        )],
        vec![TransactionOutput::new(0, Default::default())],
        Default::default(),
        Default::default(),
        Default::default(),
        Default::default(),
    )
}

pub fn run_test() {
    let rcv_owner_address = Address::new(Prefix::Testnet, Version::PubKey, &[0u8; 32]);

    // Demo deploy krc-20
    let redeem_script = build_test_inscription_redeem_script(
        build_deploy_json_example().as_bytes(),
        rcv_owner_address.clone(),
    )
    .unwrap();

    let script_public_key = pay_to_script_hash_script(&redeem_script);

    // // Create transaction
    let tx = create_spending_transaction(redeem_script, script_public_key.clone());
    let entry = UtxoEntry::new(0, script_public_key.clone(), 0, true);
    let populated_tx = PopulatedTransaction::new(&tx, vec![entry]);

    log_info!("----------------------------");
    log_info!("");

    log_info!("Tx {:?}", populated_tx.tx());

    log_info!("----------------------------");
    log_info!("");

    populated_tx.tx().inputs.iter().for_each(|input| {
        detect_krc20(&input.signature_script);
    });

    // detect_krc20(&populated_tx.tx().inputs);
}

pub fn test() {
    let test_address = Address::new(Prefix::Testnet, Version::PubKey, &[0u8; 32]);

    let test_types = [
        build_deploy_json_example(),
        build_mint_json_example(),
        build_transfer_json_example(),
    ];

    for test_script in test_types {
        if let Ok(redeem_script) =
            build_test_inscription_redeem_script(test_script.as_bytes(), test_address.clone())
        {
            let script_public_key = pay_to_script_hash_script(&redeem_script);
            println!();
            println!();
            println!("Test script:");
            println!();
            println!("{}", test_script);
            println!("---------------------------------------------------------");

            println!("^");
            println!("Redeem script public key: {:?}", script_public_key);

            // Commit transaction is P2SH.
            let is_p2sh = ScriptClass::is_pay_to_script_hash(script_public_key.script());
            if is_p2sh {
                println!("✓ - Script is_p2sh test passed");
            } else {
                println!("x - Script is_p2sh test failed");
            }

            // Reveal transaction is P2PK. Update: multisig allowed on June 3, 24, meaning P2SH.
            // Doc: https://docs.kasplex.org/protocols/krc-20-tokens/basic-operation/deploy
            let is_p2pk = ScriptClass::is_pay_to_pubkey(script_public_key.script());
            if is_p2pk {
                println!("x - Script not is_p2pk test failed");
            } else {
                println!("✓ - Script not is_p2pk test passed");
            }

            // let signature_script_ops = get_sig_op_count::<PopulatedTransaction>(&redeem_script, &script_public_key);
            // if signature_script_ops.is_empty() || signature_script_ops.iter().any(|op| op.is_err() || !op.as_ref().unwrap().is_push_opcode()) {
            //     return 0;
            // }
            // println!(
            //     "ScriptSig ops: {:02X?}",
            //     signature_script_ops
            // );

            let extracted =
                extract_script_pub_key_address(&script_public_key, "kaspatest".try_into().unwrap());
            println!();
            println!("For commit transaction");
            println!("=======================");
            println!("Output: OP_BLAKE2B <RedeemScriptHash> OP_EQUAL");

            println!(
                "Extracted script pubkey address testnet: {:?}",
                extracted.unwrap()
            );
            println!();

            println!("For reveal transaction");
            println!("=======================");
            println!();
            println!(
                "Deploy redeem script envelope in hex: {:02X?}",
                redeem_script
            );
            println!();
            println!(
                "Deploy redeem script envelope in ASCII-like: {:?}",
                bytes_as_string_format(&redeem_script[..])
            );
            println!();
        }
    }
}
