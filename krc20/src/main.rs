use krc20::optypes::KrcTwentyOpType;
use std::str::FromStr;
use krc20::transactions::test;

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
    test();
}
