use anchor_lang::prelude::*;

use crate::{EasyHavenErrors, states::*};

#[derive(Accounts)]
pub struct ApproveProperty<'info> {
    #[account(
        mut,
        has_one = user_key
    )]
    pub user: Account<'info, User>,

    #[account(
        mut,
        has_one = user_key
    )]
    pub property: Account<'info, Property>,

    #[account(mut)]
    pub user_key: Signer<'info>,

    pub system_program: Program<'info, System>
}

impl<'info> ApproveProperty<'info> {
    pub fn approve_property(
        &mut self
    ) -> Result<()> {
        require!(self.user.user_type == UserType::Owner, EasyHavenErrors::NotAOwner);

        // update owners approved key
        self.property.approved = true;

        Ok(())
    }
}