use crate::constants::PROTOCOL_ID;
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize)]
struct DeployData {
    p: String,
    op: String,
    tick: String,
    max: u64,
    lim: u64,
}

#[warn(dead_code)]
pub fn build_deploy_json_example() -> String {
    let deploy = DeployData {
        p: PROTOCOL_ID.to_string(),
        op: "deploy".to_owned(),
        tick: "test".to_owned(),
        max: 21_000,
        lim: 100,
    };
    serde_json::to_string(&deploy).unwrap()
}
