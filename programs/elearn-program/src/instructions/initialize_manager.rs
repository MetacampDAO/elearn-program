use anchor_lang::prelude::*;
use crate::states::*;
use solana_program::pubkey;
use common::errors::ErrorCode;

#[derive(Accounts)]
pub struct InitializeManager<'info> {
  // master owner
  #[account(mut,
    address=pubkey!("JakevMAR7R4Evr4PZpTqNAb1KkVXAskzYohd1vxEUqj") @ ErrorCode::UnauthorizedMaster)]
  pub master: Signer<'info>,
  // associated manager pda
  #[account(
    init, 
    seeds = [MANAGER_PDA_SEED, master.to_account_info().key.as_ref()],
    bump,
    payer = master,
    space = 8 + MAX_MANAGER_LEN
  )]
  pub manager_pda: Account<'info, Manager>,
  pub system_program: Program<'info, System>,
}

impl <'info> InitializeManager <'info> {

}

pub fn handler(
  ctx: Context<InitializeManager>,
  manager_bump: u8,
) -> Result<()> {
  ctx.accounts.manager_pda.initialize(ctx.accounts.master.to_account_info().key(), manager_bump);
  let full_permission  = PermissionType::NORMAL
    | PermissionType::CERT
    | PermissionType::BATCH
    | PermissionType::ADMIN;
  ctx.accounts.manager_pda.set_type(full_permission);
  Ok(())
}