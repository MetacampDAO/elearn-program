use anchor_lang::prelude::*;

pub const LATEST_BATCH_VERSION: u16 = 0;

pub const BATCH_PDA_SEED: &[u8] = b"batch-seed";

pub const MAX_BATCH_NAME_LEN: usize = 64;

pub const MAX_BATCH_LEN: usize = 2 // version
  + 32  // manager pubkey
  + 8   // total associated certificates
  + 8   // batch number
  + MAX_BATCH_NAME_LEN
  + 1;  // bump

#[account]
pub struct Batch {
  pub version: u16,
  pub manager_key: Pubkey,
  pub certificate_count: u64,
  pub batch_num: u64,
  pub batch_name: String,
  pub batch_bump: u8,
}

impl Batch {
  pub fn initialize(&mut self, 
    manager_key: Pubkey, 
    batch_num: u64, 
    batch_name: String, 
    batch_bump: u8
  ){
    self.version = LATEST_BATCH_VERSION;
    self.manager_key = manager_key;
    self.certificate_count = 0;
    self.batch_num = batch_num;
    self.batch_name = batch_name;
    self.batch_bump = batch_bump;
  }
}