use crate::constants::KASPLEX_HEADER;
// use crate::constants::PROTOCOL_NAMESPACE;

use crate::operations::deserialize;

// use kaspa_addresses::{Address, Prefix, Version};
use kaspa_consensus_core::tx::{PopulatedTransaction, VerifiableTransaction};
// use kaspa_txscript::get_sig_op_count;
use std::ascii::escape_default;
use std::str;

use itertools::Itertools;
use kaspa_txscript::opcodes::{deserialize_next_opcode, OpCodeImplementation};
use kaspa_txscript_errors::TxScriptError;

fn find(haystack: &[u8], needle: &Vec<u8>) -> Option<usize> {
    for (position, window) in haystack.windows(needle.len()).enumerate() {
        if window == needle {
            return Some(position);
        }
    }
    None
}

fn bytes_as_string_format(bs: &[u8]) -> String {
    let mut visible = String::new();
    for &b in bs {
        let part: Vec<u8> = escape_default(b).collect();
        visible.push_str(str::from_utf8(&part).unwrap());
    }
    visible
}

fn parse_script<T: VerifiableTransaction>(
    script: &[u8],
) -> impl Iterator<Item = Result<Box<dyn OpCodeImplementation<T>>, TxScriptError>> + '_ {
    script.iter().batching(|it| deserialize_next_opcode(it))
}

// enum TypeTx{
//     Transaction,
//     RpcTransaction
// }

pub fn detect_krc20(signature_script: &[u8]) -> bool {
    let kasplex_header = KASPLEX_HEADER.to_vec();

    let krc20_beacon: [u8; 6] = [107, 114, 99, 45, 50, 48];

    if find(signature_script, &kasplex_header).is_some() {
        println!("✓ - Kasplex header found in demo tx test passed");
        println!();
        // println!(
        //     "Deploy redeem script envelope in ASCII-like: {:?}",
        //     bytes_as_string_format(&txinputs[0].signature_script)
        // );
        println!();

        let _script_result: Result<(), TxScriptError> = parse_script(signature_script)
            .try_for_each(|opcode| {
                let opcode: Box<dyn OpCodeImplementation<PopulatedTransaction>> = opcode?;
                println!(
                    "Iter get_data OpCode: {:?}",
                    bytes_as_string_format(opcode.get_data())
                );
                println!("Iter raw OpCode: {:?}", opcode);
                println!("Iter is_conditional OpCode: {:?}", opcode.is_conditional());
                println!("Iter is_push_opcode OpCode: {:?}", opcode.is_push_opcode());
                println!("Iter value OpCode: {:?}", opcode.value());
                println!("Iter is_empty OpCode: {:?}", opcode.is_empty());
                println!();

                if !opcode.is_empty()
                    && opcode.is_push_opcode()
                    && find(opcode.get_data(), &krc20_beacon.to_vec()).is_some()
                {
                    print!("");
                    print!("Found krc-20 beacon");
                    print!("");
                    if let Some(data) = deserialize(opcode.get_data()) {
                        print!("Deserialized data: {:?}", data);
                    }

                    print!("");
                }
                if opcode.is_disabled() {
                    return Err(TxScriptError::OpcodeDisabled(format!("{:?}", opcode)));
                }

                if opcode.always_illegal() {
                    return Err(TxScriptError::OpcodeReserved(format!("{:?}", opcode)));
                }

                Ok(())
            });
    }

    false
}
