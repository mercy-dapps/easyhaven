use anchor_lang::prelude::*;

use crate::{EasyHavenErrors, states::*};

#[derive(Accounts)]
pub struct RateProperty<'info> {
    #[account(
        has_one = user_key
    )]
    pub user: Account<'info, User>,

    #[account(
        mut
    )]
    pub property: Account<'info, Property>,

    #[account(mut)]
    pub user_key: Signer<'info>,

    pub system_program: Program<'info, System>
}

impl<'info> RateProperty<'info> {
    pub fn rate_property(
        &mut self,
        rate: u8
    ) -> Result<()> {
        require!(self.user.user_type == UserType::Buyer, EasyHavenErrors::NotABuyer);
        require!(self.property.rate.len() < 10, EasyHavenErrors::MaxLengthReached);
        require!(self.property.user_key != self.user_key.key(), EasyHavenErrors::RestrictedAction);

        self.property.rate.push(rate);

        Ok(())
    }
}