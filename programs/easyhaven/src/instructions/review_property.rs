use anchor_lang::prelude::*;

use crate::{EasyHavenErrors, states::*};

#[derive(Accounts)]
pub struct ReviewProperty<'info> {
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

impl<'info> ReviewProperty<'info> {
    pub fn review_property(
        &mut self,
        text: String
    ) -> Result<()> {
        require!(self.user.user_type == UserType::Buyer, EasyHavenErrors::NotABuyer);
        require!(text.len() > 10, EasyHavenErrors::TextTooLong);
        require!(self.property.user_key != self.user_key.key(), EasyHavenErrors::RestrictedAction);


        self.property.reviews.push(Review {
            author: self.user_key.key(),
            text,
            timestamp: Clock::get()?.unix_timestamp,
        });

        Ok(())
    }
}