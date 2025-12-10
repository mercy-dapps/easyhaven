use anchor_lang::prelude::*;

use crate::{EasyHavenErrors, states::*};

#[derive(Accounts)]
#[instruction(seed: u64)]
pub struct CreateProperty<'info> {
    #[account(
        mut,
        has_one = user_key
    )]
    pub user: Account<'info, User>,

    #[account(
        mut,
        has_one = user_key
    )]
    pub owner: Account<'info, OwnerInfo>,

    #[account(
        init,
        payer = user_key,
        space = 8 + Property::INIT_SPACE,
        seeds = [b"property", user_key.key().as_ref(), seed.to_le_bytes().as_ref()],
        bump
    )]
    pub property: Account<'info, Property>,

    #[account(mut)]
    pub user_key: Signer<'info>,

    pub system_program: Program<'info, System>
}

impl<'info> CreateProperty<'info> {
    pub fn create_property(
        &mut self,
        seed: u64,
        name: String,
        details: String,
        price: u32,
        location: String,
        property_type: PropertyType,
        bumps: &CreatePropertyBumps
    ) -> Result<()> {
        require!(self.user.user_type == UserType::Owner, EasyHavenErrors::NotAOwner);

        self.property.set_inner(Property { 
            user_key: self.user_key.key(), 
            seed,
            name, 
            details, 
            price, 
            location,
            property_type,
            bump: bumps.property,
            ..Default::default()
        });

        // update owners listing count
        self.owner.listings += 1;

        Ok(())
    }
}