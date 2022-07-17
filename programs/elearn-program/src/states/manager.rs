use anchor_lang::prelude::*;
use common::errors::ErrorCode;

pub const MANAGER_PROOF_SEED: &[u8] = b"manager-seed";

pub const MAX_MANAGER_LEN: usize = 32 // manager pubkey
  + 8   // total batches created
  + 16  // total certificates created
  + 1   // permission_type
  + 1;  // bump

#[account]
pub struct Manager {
  pub manager_key: Pubkey,
  pub batch_count: u64,
  pub certificate_count: u128,
  pub permission_type: u8,
  pub manager_bump: u8
}

bitflags::bitflags! {
    pub struct PermissionType: u8 {
        const NORMAL = 1 << 0; // can view (no special privilege)
        const CERT = 1 << 1;  // can create certificate
        const BATCH = 1 << 2;   // can create batches
        const ADMIN = 1 << 3;  // can add other admins
    }
}

impl Manager {
  
  pub fn initialize(&mut self, manager_key: Pubkey, manager_bump: u8) {
    self.manager_key = manager_key;
    self.manager_bump = manager_bump;
    self.batch_count = 0;
    self.certificate_count = 0;
  }

  pub fn read_type(permission_type: u8) -> Result<PermissionType> {
    PermissionType::from_bits(permission_type).ok_or(ErrorCode::InvalidPermission.into())
  }

  pub fn set_type(&mut self, permission_type: PermissionType) {
    self.permission_type = permission_type.bits();
  }

  pub fn contains_type(&self, expected_permission_type: PermissionType) -> Result<()> {
    let permission_type = Manager::read_type(self.permission_type)?;
    if permission_type.contains(expected_permission_type) {
      msg!(
          "permission type ({:?}) matches, going ahead",
          expected_permission_type
      );
      return Ok(());
    }
    Err(ErrorCode::WrongPermission.into())
  }

}