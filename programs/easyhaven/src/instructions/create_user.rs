use anchor_lang::prelude::*;

use crate::{EasyHavenErrors, states::*};

#[derive(Accounts)]
pub struct CreateUser<'info> {
    #[account(
        init,
        payer = user_key,
        space = 8 + User::INIT_SPACE,
        seeds = [b"user", user_key.key().as_ref()],
        bump
    )]
    pub user: Account<'info, User>,

    #[account(
        init,
        payer = user_key,
        space = 8 + BuyerInfo::INIT_SPACE,
        seeds = [b"buyer", user_key.key().as_ref()],
        bump
    )]
    pub buyer: Account<'info, BuyerInfo>,

    #[account(mut)]
    pub user_key: Signer<'info>,

    pub system_program: Program<'info, System>
}

impl<'info> CreateUser<'info> {
    pub fn create_user(
        &mut self,
        name: String,
        email: String,
        phone_number: String,
        location: String,
        bumps: &CreateUserBumps
    ) -> Result<()> {
         require!(name.len() <= 50, EasyHavenErrors::NameTooLong);

        self.user.set_inner(User { 
            user_key: self.user_key.key(), 
            name, 
            email, 
            phone_number, 
            location,
            bump: bumps.user,
            ..Default::default()
        });

        // create buyer info account
        self.user.buyer_info = Some(self.buyer.key());

        self.buyer.set_inner(BuyerInfo { 
            user: self.user.user_key,
            bump: bumps.buyer,
            ..Default::default()
        });

        Ok(())
    }
}