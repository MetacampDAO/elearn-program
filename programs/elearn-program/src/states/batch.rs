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
  pub fn initialize(&mut self, manager_key: Pubkey, batch_name: String){
    self.manager_key = manager_key;
    self.certificate_count = 0;
    self.batch_name = batch_name;
  }
}