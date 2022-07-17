use anchor_lang::prelude::*;
use crate::states::*;
use common::errors::ErrorCode;

#[derive(Accounts)]
pub struct AddManager<'info> {
  #[account(mut)]
  pub admin: Signer<'info>,
  #[account(
    constraint = admin_proof.manager_key == admin.to_account_info().key()
      @ ErrorCode::ProofKeyMismatch,
    seeds = [MANAGER_PROOF_SEED, admin.to_account_info().key.as_ref()],
    bump = admin_proof.manager_bump,
  )]
  pub admin_proof: Account<'info, Manager>,
  /// CHECK: This is not dangerous because we don't read or write from this account
  pub manager_key: AccountInfo<'info>,
  #[account(
    init, 
    seeds = [MANAGER_PROOF_SEED, manager_key.key.as_ref()],
    bump,
    payer = admin,
    space = 8 + MAX_MANAGER_LEN
  )]
  pub manager_proof: Account<'info, Manager>,
  pub system_program: Program<'info, System>,
}

impl <'info> AddManager <'info> {

}

pub fn handler(
  ctx: Context<AddManager>,
  manager_bump: u8,
) -> Result<()> {
  // ensure that admin indeed has permission to add manager
  ctx.accounts.admin_proof.contains_type(PermissionType::ADMIN)?;
  ctx.accounts.manager_proof.initialize(ctx.accounts.manager_key.key(), manager_bump);
  let manager_permission  = PermissionType::NORMAL
    | PermissionType::CERT
    | PermissionType::BATCH;
  ctx.accounts.manager_proof.set_type(manager_permission);
  Ok(())
}


