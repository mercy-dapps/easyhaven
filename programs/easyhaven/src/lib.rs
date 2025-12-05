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
}
