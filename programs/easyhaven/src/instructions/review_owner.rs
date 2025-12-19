use anchor_lang::prelude::*;

use crate::{EasyHavenErrors, states::*};

#[derive(Accounts)]
pub struct ReviewOwner<'info> {
    #[account(
        has_one = user_key
    )]
    pub user: Account<'info, User>,

    #[account(
        mut
    )]
    pub owner: Account<'info, OwnerInfo>,

    #[account(mut)]
    pub user_key: Signer<'info>,

    pub system_program: Program<'info, System>
}

impl<'info> ReviewOwner<'info> {
    pub fn review_owner(
        &mut self,
        text: String
    ) -> Result<()> {
        require!(self.user.user_type == UserType::Buyer, EasyHavenErrors::NotABuyer);
        require!(text.len() > 10, EasyHavenErrors::TextTooLong);


        self.owner.reviews.push(Review {
            author: self.user_key.key(),
            text,
            timestamp: Clock::get()?.unix_timestamp,
        });

        Ok(())
    }
}