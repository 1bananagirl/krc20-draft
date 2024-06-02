use crate::constants::PROTOCOL_ID;
use crate::optypes::KrcTwentyOpType;
use serde::{Deserialize, Serialize};

#[derive(thiserror::Error, Debug)]
pub enum KrcTwentyOperationsError {
    #[error("Invalid ticker format: {0}")]
    InvalidTickerFormat(String),
}

struct Operation {
    op_type: KrcTwentyOpType,
    base_data: BaseData,
}

impl Operation {
    pub fn new(operation_type: KrcTwentyOpType, ticker: String) -> Self {
        let _validated_ticker = Self::validate_ticker(ticker.clone());
        let base_data = BaseData {
            p: PROTOCOL_ID.to_string(),
            op: operation_type.to_string(),
            tick: ticker.to_string(),
            max: None,
            lim: None,
            amt: None,
        };
        Self {
            op_type: operation_type,
            base_data,
        }
    }

    fn validate_ticker(ticker: String) -> Result<bool, KrcTwentyOperationsError> {
        let is_ascii_alpha = ticker.bytes().all(|b| b.is_ascii_lowercase());
        let is_4_chars_long = ticker.len() == 4;

        match (is_ascii_alpha, is_4_chars_long) {
            (true, true) => Ok(true),
            (_, _) => Err(KrcTwentyOperationsError::InvalidTickerFormat(ticker)),
        }
    }

    fn data(&self) -> BaseData {
        self.validate();
        self.base_data.clone()
    }

    fn set_max(&mut self, cap: u64) {
        self.base_data.max = Some(cap);
    }

    fn set_lim(&mut self, mint_limit: u64) {
        self.base_data.lim = Some(mint_limit);
    }

    fn set_amt(&mut self, transfer_amount: u64) {
        self.base_data.amt = Some(transfer_amount);
    }

    fn validate(&self) -> bool {
        match self.op_type {
            KrcTwentyOpType::Deploy => self.base_data.max.is_some() && self.base_data.lim.is_some(),
            KrcTwentyOpType::Mint => true,
            KrcTwentyOpType::Transfer => self.base_data.amt.is_some(),
        }
    }

    fn build_deploy(ticker: String, cap: u64, mint_limit: u64) -> DeployData {
        // assert!(self.op_type == KrcTwentyOpType::Deploy);
        let mut operation = Self::new(KrcTwentyOpType::Deploy, ticker);
        operation.set_max(cap);
        operation.set_lim(mint_limit);
        operation.data().into()
    }
    fn build_mint(ticker: String) -> MintData {
        let operation = Self::new(KrcTwentyOpType::Mint, ticker);
        operation.data().into()
    }
    fn build_transfer(ticker: String, transfer_amount: u64) -> TransferData {
        let mut operation = Self::new(KrcTwentyOpType::Transfer, ticker);
        operation.set_amt(transfer_amount);
        operation.data().into()
    }
}

impl From<BaseData> for TransferData {
    fn from(val: BaseData) -> Self {
        TransferData {
            p: val.p,
            op: val.op,
            tick: val.tick,
            amt: val.amt.unwrap(),
        }
    }
}

impl From<BaseData> for DeployData {
    fn from(val: BaseData) -> Self {
        DeployData {
            p: val.p,
            op: val.op,
            tick: val.tick,
            max: val.max.unwrap(),
            lim: val.lim.unwrap(),
        }
    }
}

impl From<BaseData> for MintData {
    fn from(val: BaseData) -> Self {
        MintData {
            p: val.p,
            op: val.op,
            tick: val.tick,
        }
    }
}

#[derive(Debug, Clone)]
struct BaseData {
    p: String,
    op: String,
    tick: String,
    max: Option<u64>,
    lim: Option<u64>,
    amt: Option<u64>,
}

#[derive(Serialize, Deserialize)]
struct DeployData {
    p: String,
    op: String,
    tick: String,
    max: u64,
    lim: u64,
}

#[derive(Serialize, Deserialize)]
struct MintData {
    p: String,
    op: String,
    tick: String,
}

#[derive(Serialize, Deserialize)]
struct TransferData {
    p: String,
    op: String,
    tick: String,
    amt: u64,
}

pub fn build_deploy_json_example() -> String {
    let payload = Operation::build_deploy("test".to_owned(), 21_000, 100);
    serde_json::to_string(&payload).unwrap()
}

pub fn build_mint_json_example() -> String {
    let payload = Operation::build_mint("test".to_owned());
    serde_json::to_string(&payload).unwrap()
}

pub fn build_transfer_json_example() -> String {
    let payload = Operation::build_transfer("test".to_owned(), 1_000);
    serde_json::to_string(&payload).unwrap()
}
