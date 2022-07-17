use anchor_lang::prelude::*;
use crate::states::*;
use common::errors::ErrorCode;

#[derive(Accounts)]
pub struct ModifyManager<'info> {
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
  #[account(mut,
    constraint = manager_proof.manager_key == manager_key.key()
      @ ErrorCode::ProofKeyMismatch,
    seeds = [MANAGER_PROOF_SEED, manager_key.key.as_ref()],
    bump = manager_proof.manager_bump,
  )]
  pub manager_proof: Account<'info, Manager>,
  pub system_program: Program<'info, System>,
}

impl <'info> ModifyManager <'info> {

}

pub fn handler (
  ctx: Context<ModifyManager>,
  target_permissions: u8,
) -> Result<()> {
  ctx.accounts.admin_proof.contains_type(PermissionType::ADMIN)?;
  let target_permissions = Manager::read_type(target_permissions)?;
  ctx.accounts.manager_proof.set_type(target_permissions);
  Ok(())
}





