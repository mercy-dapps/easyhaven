use anchor_lang::prelude::*;

use crate::{states::*};

#[derive(Accounts)]
pub struct BecomeAHost<'info> {
    #[account(
        mut
    )]
    pub user: Account<'info, User>
}

impl<'info> BecomeAHost<'info>  {
    pub fn become_a_host(&mut self) -> Result<()> {

        self.user.user_type = UserType::Owner;

        Ok(())
    }
}