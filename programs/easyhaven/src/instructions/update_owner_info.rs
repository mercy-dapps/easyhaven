use anchor_lang::prelude::*;

use crate::{EasyHavenErrors, states::*};

#[derive(Accounts)]
pub struct UpdateOwnerInfo<'info> {
    #[account(
        mut,
        has_one = user_key
    )]
    pub user: Account<'info, User>,

    #[account(
        mut,
    )]
    pub owner: Account<'info, OwnerInfo>,

    #[account(mut)]
    pub user_key: Signer<'info>,

    pub system_program: Program<'info, System>
}

impl<'info> UpdateOwnerInfo<'info> {
    pub fn update_owner_info(
        &mut self,
        gender: Option<Gender>,
        profile_picture: Option<String>,
        bio: Option<String>,
        profession: Option<String>,

        languages_spoken: Option<Vec<String>>,
    ) -> Result<()> {

        require!(self.user.user_type == UserType::Owner, EasyHavenErrors::NotAOwner);
        
        // update user basic info
         if let Some(g) = gender {
            self.user.gender = g;
        }

        if let Some(pic) = profile_picture {
            self.user.profile_picture = pic;
        }

        if let Some(b) = bio {
            self.user.bio = b;
        }

        if let Some(p) = profession {
            self.user.profession = p;
        }

        // update owner info
        if let Some(lang) = languages_spoken {
            self.owner.languages_spoken = lang;
        }

        Ok(())
    }
}