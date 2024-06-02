use crate::constants::{FEE_DEPLOY, FEE_MINT};
use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(thiserror::Error, Debug)]
pub enum KrcTwentyOpTypeError {
    #[error("Invalid op type: {0}")]
    InvalidOpType(String),
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
