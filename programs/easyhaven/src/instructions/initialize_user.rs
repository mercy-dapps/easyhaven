use anchor_lang::prelude::*;

use crate::{EasyHavenErrors, states::*};

#[derive(Accounts)]
pub struct InitializeUser<'info> {
    #[account(
        init,
        payer = signer,
        space = 8 + User::INIT_SPACE,
        seeds = [b"user", signer.key().as_ref()],
        bump
    )]
    pub user: Account<'info, User>,

    #[account(mut)]
    pub signer: Signer<'info>,

    pub system_program: Program<'info, System>

}


impl<'info> InitializeUser<'info> {
    pub fn create_user(
        &mut self,
        name: String,
        email: String,
        phone_number: String,
        location: String,
        bumps: &InitializeUserBumps
    ) -> Result<()> {
         require!(name.len() <= 50, EasyHavenErrors::NameTooLong);

        self.user.set_inner(User { 
            user_key: self.user.key(), 
            name, 
            email, 
            phone_number, 
            location,
            bump: bumps.user,
            ..Default::default()
        });

        Ok(())
    }
}