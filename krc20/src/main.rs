use std::fmt::{Display,Formatter};
use std::str::FromStr;

#[derive(thiserror::Error, PartialEq, Eq, Debug, Clone)]
pub enum KrcTwentyOpTypeError {
    #[error("Invalid KRC20 op type: {0}")]
    InvalidOpType(String),
}

// 1_000 KAS
pub const FEE_DEPLOY: u64 = 100000000000;
// 1 KAS
pub const FEE_MINT: u64 = 100000000;

pub enum KrcTwentyOpType  {
    Deploy,
    Mint,
    Transfer
}

impl KrcTwentyOpType {
    pub fn additional_fee(&self) -> u64 {
        match self {
            KrcTwentyOpType::Deploy => FEE_DEPLOY,
            KrcTwentyOpType::Mint => FEE_MINT,
            KrcTwentyOpType::Transfer => 0
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

fn main() {
    println!("Hello, world!");
    println!("{}", KrcTwentyOpType::Mint);
    println!("For minting: {}", KrcTwentyOpType::from_str("mint").unwrap().additional_fee());
    println!("For deploying: {}", KrcTwentyOpType::from_str("deploy").unwrap().additional_fee());
}
