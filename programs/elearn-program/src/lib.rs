use anchor_lang::prelude::*;
use instructions::*;

pub mod instructions;
pub mod states;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod elearn {
    use super::*;

    pub fn initialize_manager(
        ctx: Context<InitializeManager>,
        manager_bump: u8,
    ) -> Result<()> {
        initialize_manager::handler(ctx, manager_bump)
    }

    pub fn add_manager(
        ctx: Context<AddManager>,
        manager_bump: u8,
    ) -> Result<()> {
        add_manager::handler(ctx, manager_bump)
    }

    pub fn modify_manager(
        ctx: Context<ModifyManager>,
        target_permissions: u8,
    ) -> Result<()> {
        modify_manager::handler(ctx, target_permissions)
    }


}
