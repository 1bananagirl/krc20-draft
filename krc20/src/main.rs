use krc20::optypes::KrcTwentyOpType;
use krc20::transactions::run_test;
use std::str::FromStr;
use krc20::client::main as wrpc_client_main;

// #[tokio::main]
// async fn main() -> Result<(), Box<dyn std::error::Error> {
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
    // test();
    run_test();
    wrpc_client_main().unwrap();
    // Ok(())
}
