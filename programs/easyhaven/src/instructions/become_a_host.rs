use anchor_lang::prelude::*;

use crate::{EasyHavenErrors, states::*};

#[derive(Accounts)]
pub struct BecomeAHost<'info> {
    #[account(
        mut,
        has_one = user_key
    )]
    pub user: Account<'info, User>,

    #[account(
        init,
        payer = user_key,
        space = 8 + OwnerInfo::INIT_SPACE,
        seeds = [b"owner", user_key.key().as_ref()],
        bump
    )]
    pub owner: Account<'info, OwnerInfo>,

    #[account(mut)]
    pub user_key: Signer<'info>,

    pub system_program: Program<'info, System>
}

impl<'info> BecomeAHost<'info>  {
    pub fn become_a_host(&mut self, bumps: &BecomeAHostBumps) -> Result<()> {

        require!(self.user.user_key == self.user_key.key(), EasyHavenErrors::WrongAccount);

        self.user.user_type = UserType::Owner;

         self.user.owner_info = Some(self.owner.key());

        // create owner info account
        self.owner.set_inner(OwnerInfo { 
            user: self.user.user_key,
            bump: bumps.owner,
            ..Default::default()
        });

        Ok(())
    }
}