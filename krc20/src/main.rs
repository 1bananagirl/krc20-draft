// use krc20::client::main as wrpc_client_main;
use krc20::transactions::test_and_verify_sign;

fn main() {
    test_and_verify_sign();
    // wrpc_client_main().unwrap();
}
