use anchor_lang::prelude::*;
use crate::states::*;
use common::errors::ErrorCode;

#[derive(Accounts)]
pub struct CreateBatch<'info> {
  #[account(mut)]
  pub manager: Signer<'info>,
  #[account(mut,
    constraint = manager_proof.manager_key == manager.to_account_info().key()
      @ ErrorCode::ProofKeyMismatch,
    seeds = [MANAGER_PROOF_SEED, manager.to_account_info().key.as_ref()],
    bump = manager_proof.manager_bump,
  )]
  pub manager_proof: Account<'info, Manager>,
  #[account(
    init, 
    seeds = [
      BATCH_PDA_SEED, 
      manager.to_account_info().key.as_ref(), 
      manager_proof.batch_count.to_le_bytes().as_ref()],
    bump,
    payer = manager,
    space = 8 + MAX_BATCH_LEN
  )]
  pub batch: Account<'info, Batch>,
  pub system_program: Program<'info, System>, 
}

impl <'info> CreateBatch <'info> {

}

pub fn handler (
  ctx: Context<CreateBatch>,
  batch_num: u64,
  batch_name: String,
  batch_bump: u8
) -> Result<()> {
  ctx.accounts.manager_proof.contains_type(PermissionType::BATCH)?;
  ctx.accounts.batch.initialize(
    ctx.accounts.manager.key(),
    batch_num,
    batch_name,
    batch_bump
  );
  ctx.accounts.manager_proof.batch_count += 1;
  Ok(())
}
