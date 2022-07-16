use anchor_lang::prelude::*;

pub const BATCH_PDA_SEED: &[u8] = b"batch-seed";

pub const MAX_BATCH_NAME_LEN: usize = 64;

pub const MAX_BATCH_LEN: usize = 32 // manager pubkey
  + 8   // total associated certificates
  + MAX_BATCH_NAME_LEN;

#[account]
pub struct Batch {
  pub manager_key: Pubkey,
  pub certificate_count: u64,
  pub batch_name: String,
}

impl Batch {

}