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
    // let kasplex_header = KASPLEX_HEADER.to_vec();
    let kasplex_header: &[u8] = "kasplex".as_bytes();
    let kasplex_header_uppercase_r = "kasplex".to_ascii_uppercase();
    let kasplex_header_uppercase = kasplex_header_uppercase_r.as_bytes();

    let krc20_beacon: [u8; 6] = [107, 114, 99, 45, 50, 48];

    let krc20_uppercase_beacon = "KRC-20".as_bytes();

    if find(signature_script, &kasplex_header.to_vec()).is_some()
    ||
     find(signature_script, &kasplex_header_uppercase.to_vec()).is_some()
    {
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
                    "Iter get_data as_string OpCode: {:?}",
                    bytes_as_string_format(opcode.get_data())
                );
                println!(
                    "Iter get_data OpCode: {:?}",
                    opcode.get_data()
                );
                println!("Iter raw OpCode: {:?}", opcode);
                println!("Iter is_conditional OpCode: {:?}", opcode.is_conditional());
                println!("Iter is_push_opcode OpCode: {:?}", opcode.is_push_opcode());
                println!("Iter value OpCode: {:?}", opcode.value());
                println!("Iter is_empty OpCode: {:?}", opcode.is_empty());
                println!("Iter len OpCode: {:?}", opcode.len());
                println!();

                // Observed on TN11
                //
                // OpCode: 65 Opcode<0x41> => OpData65 = 0x41,
                //
                // Value 76 Opcode<0x4c> => OpPushData1 = 0x4c,

                if !opcode.is_empty()
                    && opcode.is_push_opcode()
                    && (find(opcode.get_data(), &krc20_beacon.to_vec()).is_some()
                        || find(opcode.get_data(), &krc20_uppercase_beacon.to_vec()).is_some())
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
            println!("-------------------------------------------------");
    }

    false
}

// Observed TN11 data reverse engineering
pub fn observed_tn11(){

    // 0x41 data
    let observed_op_data65:[u8; 65] = [104, 156, 143, 162, 42, 151, 189, 142, 191, 189, 50, 199, 200, 143, 148, 101, 96, 46, 229, 246, 45, 119, 198, 222, 15, 9, 243, 187, 9, 241, 226, 152, 79, 188, 88, 138, 32, 209, 34, 117, 228, 78, 29, 86, 248, 150, 230, 109, 162, 126, 18, 180, 67, 85, 125, 156, 93, 186, 211, 226, 126, 93, 248, 197, 1];

    // 0x4c data or OpPushData2 also supported
    let observed_op_pushdata1:[u8; 91] = [32, 138, 214, 99, 52, 105, 85, 129, 55, 75, 214, 225, 186, 91, 113, 13, 54, 86, 144, 215, 167, 88, 83, 88, 82, 254, 130, 35, 222, 236, 197, 65, 231, 172, 0, 99, 7, 107, 97, 115, 112, 108, 101, 120, 81, 1, 0, 0, 41, 123, 34, 112, 34, 58, 34, 75, 82, 67, 45, 50, 48, 34, 44, 34, 111, 112, 34, 58, 34, 109, 105, 110, 116, 34, 44, 34, 116, 105, 99, 107, 34, 58, 34, 111, 106, 98, 107, 111, 34, 125, 104];

    println!("{:x?}", observed_op_data65);
    println!("{:x?}", observed_op_pushdata1);
    
}