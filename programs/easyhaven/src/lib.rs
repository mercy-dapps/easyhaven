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

    pub fn initialize_owner(
        ctx: Context<InitializeOwner>, 
        name: String,
        email: String,
        phone_number: String,
        location: String,
     ) -> Result<()> {
        ctx.accounts.create_owner(
            name,
            email,
            phone_number,
            location,
            &ctx.bumps
        )?;

        Ok(())

    }

    pub fn initialize_user(
        ctx: Context<InitializeUser>, 
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
        ctx.accounts.become_a_host()?;

        Ok(())
    }

    pub fn update_user(
        ctx: Context<UpdateUser>,
        gender: Option<Gender>,
        profile_picture: Option<String>,
        bio: Option<String>,
        profession: Option<String>,
        languages_spoken: Option<Vec<String>>,
        budgets: Option<u32>
    ) -> Result<()> {
        ctx.accounts.update_user(
            gender,
            profile_picture,
            bio,
            profession,
            languages_spoken,
            budgets
        )?;

        Ok(())
    }
}
