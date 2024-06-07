use crate::{
    constants::KASPLEX_HEADER_LC, constants::KASPLEX_HEADER_UC, constants::KRC20_HEADER_LC,
    constants::KRC20_HEADER_UC, operations::BaseData,
};

use crate::operations::deserialize;
use kaspa_consensus_core::tx::{PopulatedTransaction, Transaction, VerifiableTransaction};
use kaspa_rpc_core::RpcTransaction;

use itertools::Itertools;
use kaspa_txscript::opcodes::{deserialize_next_opcode, OpCodeImplementation};
use kaspa_txscript_errors::TxScriptError;

fn window_find(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    for (position, window) in haystack.windows(needle.len()).enumerate() {
        if window == needle {
            return Some(position);
        }
    }
    None
}

fn parse_script<T: VerifiableTransaction>(
    script: &[u8],
) -> impl Iterator<Item = Result<Box<dyn OpCodeImplementation<T>>, TxScriptError>> + '_ {
    script.iter().batching(|it| deserialize_next_opcode(it))
}

pub trait ITransaction {
    fn signature_script(&self) -> &[u8];
}

impl ITransaction for &RpcTransaction {
    fn signature_script(&self) -> &[u8] {
        &self.inputs[0].signature_script[..]
    }
}

impl ITransaction for &Transaction {
    fn signature_script(&self) -> &[u8] {
        &self.inputs[0].signature_script[..]
    }
}

pub fn detect_krc20_header(haystack: &[u8]) -> bool {
    window_find(haystack, &KRC20_HEADER_UC).is_some()
        || window_find(haystack, &KRC20_HEADER_LC).is_some()
}

pub fn detect_kasplex_header(haystack: &[u8]) -> bool {
    window_find(haystack, &KASPLEX_HEADER_LC).is_some()
        || window_find(haystack, &KASPLEX_HEADER_UC).is_some()
}

pub fn detect_krc20<T: ITransaction>(sigtx: T) -> Option<BaseData> {
    let signature_script = &sigtx.signature_script();

    let mut inscription: Option<BaseData> = None;

    if detect_kasplex_header(signature_script) {
        // Second OpCode only lookup optimization.
        let mut op_position = 0;

        let _script_result: Result<(), TxScriptError> = parse_script(signature_script)
            .try_for_each(|opcode| {
                let opcode: Box<dyn OpCodeImplementation<PopulatedTransaction>> = opcode?;

                op_position += 1;

                if op_position != 2 {
                    ()
                }

                if !opcode.is_empty()
                    && opcode.is_push_opcode()
                    && detect_krc20_header(opcode.get_data())
                {
                    // Second-to-last only lookup optimization.
                    let mut previous: Option<Vec<u8>> = None;
                    let mut current: Option<Vec<u8>> = None;

                    let _result: Result<(), TxScriptError> = parse_script(opcode.get_data())
                        .try_for_each(
                            |inner_opcode: Result<
                                Box<dyn OpCodeImplementation<PopulatedTransaction>>,
                                TxScriptError,
                            >| {
                                let inner_opcode: Box<
                                    dyn OpCodeImplementation<PopulatedTransaction>,
                                > = inner_opcode?;
                                previous.clone_from(&current);
                                current = match !inner_opcode.is_empty()
                                    && inner_opcode.is_push_opcode()
                                {
                                    true => Some(inner_opcode.get_data().to_vec()),
                                    false => None,
                                };
                                Ok(())
                            },
                        );

                    if previous.is_some() {
                        let second_to_last_op = &previous.unwrap()[..];

                        if detect_krc20_header(opcode.get_data()) {
                            if let Some(data) = deserialize(second_to_last_op) {
                                inscription = Some(data);
                            }
                        }
                    }
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

    inscription
}
