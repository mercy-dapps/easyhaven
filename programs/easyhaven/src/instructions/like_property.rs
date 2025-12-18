use anchor_lang::prelude::*;

use crate::{EasyHavenErrors, states::*};

#[derive(Accounts)]
#[instruction(seed: u64)]
pub struct LikeProperty<'info> {
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

impl<'info> LikeProperty<'info> {
    pub fn like_property(
        &mut self,
        seed: u64
    ) -> Result<()> {
        require!(self.property.liked_pubkey.len() < 10, EasyHavenErrors::MaxLengthReached);
        require!(self.property.seed == seed, EasyHavenErrors::InvalidData);


        self.property.liked_pubkey.push(self.user.user_key);
        self.property.liked_count.checked_add(1).unwrap();

        Ok(())
    }
}