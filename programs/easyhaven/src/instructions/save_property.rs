use anchor_lang::prelude::*;

use crate::{EasyHavenErrors, states::*};

#[derive(Accounts)]
#[instruction(seed: u64)]
pub struct SaveProperty<'info> {
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

impl<'info> SaveProperty<'info> {
    pub fn save_property(
        &mut self,
        seed: u64 
    ) -> Result<()> {
        require!(self.property.seed == seed, EasyHavenErrors::InvalidData);
        require!(self.property.saved_pubkey.len() < 10, EasyHavenErrors::MaxLengthReached);

        self.property.saved_pubkey.push(self.user.user_key);

        Ok(())
    }
}