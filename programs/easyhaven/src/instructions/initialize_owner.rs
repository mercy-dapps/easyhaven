use anchor_lang::prelude::*;

use crate::{EasyHavenErrors, states::*};

#[derive(Accounts)]
pub struct InitializeOwner<'info> {
    #[account(
        init,
        payer = owner,
        space = 8 + PropertyOwner::INIT_SPACE,
        seeds = [b"owner", owner.key().as_ref()],
        bump
    )]
    pub property_owner: Account<'info, PropertyOwner>,

    #[account(mut)]
    pub owner: Signer<'info>,

    pub system_program: Program<'info, System>

}

impl<'info> InitializeOwner<'info> {
    pub fn create_owner(
        &mut self,
        name: String,
        email: String,
        phone_number: String,
        location: String,
        bumps: &InitializeOwnerBumps
    ) -> Result<()> {
         require!(name.len() <= 50, EasyHavenErrors::NameTooLong);

        self.property_owner.set_inner(PropertyOwner { 
            owner: self.owner.key(), 
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