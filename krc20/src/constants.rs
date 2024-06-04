use kaspa_consensus_core::constants::SOMPI_PER_KASPA;

pub const FEE_DEPLOY: u64 = 1_000 * SOMPI_PER_KASPA;
pub const FEE_MINT: u64 = SOMPI_PER_KASPA;
pub const PROTOCOL_NAMESPACE: &str = "kasplex";
pub const PROTOCOL_ID: &str = "krc-20";
pub const KASPLEX_HEADER: &[u8; 7] = b"\x6b\x61\x73\x70\x6c\x65\x78"; // kasplex in hex 6B 61 73 70 6C 65 78
                                                                      // pub const KASPLEX_HEADER: &[u8; 10] = b"\x00\x63\x07\x6b\x61\x73\x70\x6c\x65\x78"; // kasplex in hex 6B 61 73 70 6C 65 78
