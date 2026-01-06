use anchor_lang::prelude::*;

use crate::{EasyHavenErrors, states::*};

#[derive(Accounts)]
#[instruction(seed: u64)]
pub struct EditProperty<'info> {
    #[account(
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

impl<'info> EditProperty<'info> {
    pub fn edit_property(
        &mut self,
        seed:u64,
        name:  Option<String>,
        details: Option<String>,
        mode_of_payment: PaymentMode,
    ) -> Result<()> {
        require!(self.user.user_type == UserType::Owner, EasyHavenErrors::NotAOwner);
        require!(self.user.user_key == self.user_key.key(), EasyHavenErrors::WrongAccount);
        require!(self.property.seed == seed, EasyHavenErrors::WrongProperty);

        // update user basic info
        if let Some(name) = name {
            self.property.name = name;
        }

        if let Some(details) = details {
            self.property.details = details;
        }

        self.property.mode_of_payment = mode_of_payment;

        Ok(())
    }
}