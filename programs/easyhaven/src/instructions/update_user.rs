use anchor_lang::prelude::*;

use crate::{EasyHavenErrors, states::*};

#[derive(Accounts)]
pub struct UpdateUser<'info> {
    #[account(
        mut
    )]
    pub user: Account<'info, User>
}


impl<'info> UpdateUser<'info> {
    pub fn update_user(
        &mut self,
        gender: Option<Gender>,
        profile_picture: Option<String>,
        bio: Option<String>,
        profession: Option<String>,
        languages_spoken: Option<Vec<String>>,
        budgets: Option<u32>
    ) -> Result<()> {
         if let Some(g) = gender {
            self.user.gender = g;
        }

         // Update profile picture if provided
        if let Some(pic) = profile_picture {
            self.user.profile_picture = pic;
        }

        // Update bio and profession if provided
        if let Some(b) = bio {
            self.user.bio = b;
        }

        if let Some(p) = profession {
            self.user.profession = p;
        }

        // Update owner info partially
        if self.user.user_type == UserType::Owner {
            if let Some(languages) = languages_spoken {
                let mut owner_info = self.user.owner_info.clone().unwrap_or_default();
                owner_info.languages_spoken = languages;
                self.user.owner_info = Some(owner_info);
            }
        }

        // Update buyer info partially
        if self.user.user_type == UserType::Owner {
            if let Some(bud) = budgets {
                let mut buyer_info = self.user.buyer_info.clone().unwrap_or_default();
                buyer_info.budgets = bud;
                self.user.buyer_info = Some(buyer_info);
            }
        }

     Ok(())
    }
}