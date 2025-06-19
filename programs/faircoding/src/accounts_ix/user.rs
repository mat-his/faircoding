use anchor_lang::prelude::*;

use crate::state::User;

#[derive(Accounts)]
pub struct CreateUserAccount<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        init,
        payer = payer,
        space = 8 + User::INIT_SPACE,
        seeds = [b"user", owner.key().as_ref()],
        bump)]
    pub user: Account<'info, User>,
    pub system_program: Program<'info, System>,
}
