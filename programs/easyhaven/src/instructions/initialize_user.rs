use anchor_lang::prelude::*;

use crate::{EasyHavenErrors, states::*};

#[derive(Accounts)]
pub struct InitializeUser<'info> {
    #[account(
        init,
        payer = user,
        space = 8 + PropertyOwner::INIT_SPACE,
        seeds = [b"user", user.key().as_ref()],
        bump
    )]
    pub property_owner: Account<'info, PropertyOwner>,

    #[account(mut)]
    pub user: Signer<'info>,

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

        self.property_owner.set_inner(PropertyOwner { 
            owner: self.user.key(), 
            name, 
            email, 
            phone_number, 
            location, 
            bump: bumps.property_owner,
            ..Default::default()
        });

        Ok(())
    }
}