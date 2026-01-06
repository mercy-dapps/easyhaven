use anchor_lang::prelude::*;

use crate::{EasyHavenErrors, states::*};

#[derive(Accounts)]
#[instruction(seed: u64)]
pub struct LikeProperty<'info> {
    #[account(
        mut
    )]
    pub property: Account<'info, Property>,

    #[account(mut)]
    pub user_key: Signer<'info>,

    pub system_program: Program<'info, System>
}

impl<'info> LikeProperty<'info> {
    pub fn like_property(
        &mut self,
        seed: u64
    ) -> Result<()> {
        require!(self.property.liked_pubkey.len() < 10, EasyHavenErrors::MaxLengthReached);
        require!(self.property.seed == seed, EasyHavenErrors::InvalidData);
        require!(self.property.user_key != self.user_key.key(), EasyHavenErrors::RestrictedAction);

        self.property.liked_pubkey.push(self.user_key.key().clone());

        Ok(())
    }
}