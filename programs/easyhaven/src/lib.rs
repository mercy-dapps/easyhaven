pub mod states;
pub mod error;
pub mod instructions;

use anchor_lang::prelude::*;

pub use states::*;
pub use error::*;
pub use instructions::*;


declare_id!("DpBVVak7xw7kb2de3xzADGJ6tBgf8vkGKtXCCNtTMXey");

#[program]
pub mod easyhaven {
    use super::*;

    pub fn create_user(
        ctx: Context<CreateUser>, 
        name: String,
        email: String,
        phone_number: String,
        location: String,
    ) -> Result<()> {
        ctx.accounts.create_user(
            name,
            email,
            phone_number,
            location,
            &ctx.bumps
        )?;

        Ok(())
    }

    pub fn become_a_host(ctx: Context<BecomeAHost>) -> Result<()> {
        ctx.accounts.become_a_host(&ctx.bumps)?;

        Ok(())
    }

    pub fn update_buyer_info(
        ctx: Context<UpdateBuyerInfo>,
        gender: Option<Gender>,
        profile_picture: Option<String>,
        bio: Option<String>,
        profession: Option<String>,

        interest_properties: Option<Vec<String>>,
        locations_preferred: Option<Vec<String>>,
        budgets: Option<u32>
    ) -> Result<()> {
        ctx.accounts.update_buyer_info(
            gender,
            profile_picture,
            bio,
            profession,

            interest_properties,
            locations_preferred,
            budgets
        )?;

        Ok(())
    }

    pub fn update_owner_info(
        ctx: Context<UpdateOwnerInfo>,
        gender: Option<Gender>,
        profile_picture: Option<String>,
        bio: Option<String>,
        profession: Option<String>,

        languages_spoken: Option<Vec<String>>
    ) -> Result<()> {
        ctx.accounts.update_owner_info(
            gender,
            profile_picture,
            bio,
            profession,

            languages_spoken
        )?;

        Ok(())
    }
}
