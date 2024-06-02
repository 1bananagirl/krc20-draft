use krc20::optypes::KrcTwentyOpType;
use krc20::transactions::test;
use std::str::FromStr;

fn main() {
    println!();
    println!("KRC-20 rust draft");
    println!("=======================");
    // println!();
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
    test();
}
