use anchor_lang::prelude::*;
use crate::states::*;
use common::errors::ErrorCode;

#[derive(Accounts)]
pub struct CreateCertificate<'info> {
  #[account(mut)]
  pub manager: Signer<'info>,
  #[account(mut,
    constraint = manager_proof.manager_key == manager.to_account_info().key()
      @ ErrorCode::ProofKeyMismatch,
    seeds = [MANAGER_PROOF_SEED, manager.to_account_info().key.as_ref()],
    bump = manager_proof.manager_bump,
  )]
  pub manager_proof: Account<'info, Manager>,
  #[account(mut,
    constraint = batch.manager_key == manager.to_account_info().key()
      @ ErrorCode::BatchKeyMismatch,
    seeds = [
      BATCH_PDA_SEED, 
      manager.to_account_info().key.as_ref(), 
      batch.batch_num.to_le_bytes().as_ref()],
    bump = batch.batch_bump
  )]
  pub batch: Account<'info, Batch>,
  #[account(
    init, 
    seeds = [
      CERTIFICATE_PDA_SEED, 
      batch.to_account_info().key.as_ref(), 
      batch.certificate_count.to_le_bytes().as_ref()],
    bump,
    payer = manager,
    space = 8 + MAX_CERTIFICATE_LEN
  )]
  pub certificate: Account<'info, Certificate>,
  /// CHECK: This is not dangerous because we don't read or write from this account
  pub student_key: AccountInfo<'info>,
  pub system_program: Program<'info, System>,
}

impl <'info> CreateCertificate <'info> {

}

pub fn handler (
  ctx: Context<CreateCertificate>,
  complete_date: u64,
  certificate_bump: u8,
  student_name: String,
  student_grade: String,
  course_name: String,
  school_name: String,
  school_uri: String,
  issuer_name: String,
  issuer_role: String,
  issuer_uri: String,
) -> Result<()> {
  ctx.accounts.manager_proof.contains_type(PermissionType::CERT)?;
  ctx.accounts.certificate.initialize(
    ctx.accounts.batch.to_account_info().key(), 
    ctx.accounts.manager.key(), 
    ctx.accounts.student_key.key(), 
    complete_date, 
    ctx.accounts.batch.certificate_count, 
    certificate_bump, 
    student_name, 
    student_grade, 
    course_name, 
    school_name, 
    school_uri, 
    issuer_name, 
    issuer_role, 
    issuer_uri);
  ctx.accounts.manager_proof.certificate_count += 1;
  ctx.accounts.batch.certificate_count += 1;
  Ok(())
}