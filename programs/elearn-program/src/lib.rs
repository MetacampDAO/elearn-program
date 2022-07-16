use anchor_lang::prelude::*;
use instructions::*;

pub mod instructions;
pub mod states;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod elearn_program {
    use super::*;

    pub fn initialize_manager(
        ctx: Context<InitializeManager>,
        manager_bump: u8,
    ) -> Result<()> {
        initialize_manager::handler(ctx, manager_bump)
    }
}