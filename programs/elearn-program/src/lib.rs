use anchor_lang::prelude::*;
use instructions::*;

pub mod instructions;
pub mod states;

declare_id!("9UZhEPPrzXSAScvZ2cDF1GgYGJTZrAcPUEwNGLvvu9Pr");

#[program]
pub mod elearn {
    use super::*;

    pub fn initialize_manager(ctx: Context<InitializeManager>, manager_bump: u8) -> Result<()> {
        initialize_manager::handler(ctx, manager_bump)
    }

    pub fn add_manager(ctx: Context<AddManager>, manager_bump: u8) -> Result<()> {
        add_manager::handler(ctx, manager_bump)
    }

    pub fn modify_manager(ctx: Context<ModifyManager>, target_permissions: u8) -> Result<()> {
        modify_manager::handler(ctx, target_permissions)
    }

    pub fn create_batch(
        ctx: Context<CreateBatch>,
        batch_name: String,
        batch_bump: u8,
    ) -> Result<()> {
        create_batch::handler(ctx, batch_name, batch_bump)
    }

    pub fn create_certificate(
        ctx: Context<CreateCertificate>,
        start_date: u64,
        end_date: u64,
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
        create_certificate::handler(
            ctx,
            start_date,
            end_date,
            complete_date,
            certificate_bump,
            student_name,
            student_grade,
            course_name,
            school_name,
            school_uri,
            issuer_name,
            issuer_role,
            issuer_uri,
        )
    }
}
