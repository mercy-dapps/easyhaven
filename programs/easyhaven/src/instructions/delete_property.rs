use anchor_lang::prelude::*;

use crate::{EasyHavenErrors, states::*};

#[derive(Accounts)]
#[instruction(seed: u64)]
pub struct DeleteProperty<'info> {
    #[account(
        has_one = user_key
    )]
    pub user: Account<'info, User>,

    #[account(
        mut,
        has_one = user_key
    )]
    pub owner: Account<'info, OwnerInfo>,

    #[account(
        mut,
        close = user_key
    )]
    pub property: Account<'info, Property>,

    #[account(mut)]
    pub user_key: Signer<'info>,

    pub system_program: Program<'info, System> // not sure we need this
}

impl<'info> DeleteProperty<'info> {
    pub fn delete_property(
        &mut self
    ) -> Result<()> {
        require!(self.user.user_type == UserType::Owner, EasyHavenErrors::NotAOwner);
        require!(self.user.user_key == self.user_key.key(), EasyHavenErrors::WrongAccount);
 
        self.owner.listings -= 1;

        Ok(())
    }
}