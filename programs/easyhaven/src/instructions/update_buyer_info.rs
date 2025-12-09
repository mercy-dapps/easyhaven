use anchor_lang::prelude::*;

use crate::{EasyHavenErrors, states::*};

#[derive(Accounts)]
pub struct UpdateBuyerInfo<'info> {
    #[account(
        mut,
        has_one = user_key 
    )]
    pub user: Account<'info, User>,

    #[account(
        mut,
        seeds = [b"buyer", user_key.key().as_ref()],
        bump = buyer.bump
    )]
    pub buyer: Account<'info, BuyerInfo>,

    #[account(mut)]
    pub user_key: Signer<'info>,

    pub system_program: Program<'info, System>
}

impl<'info> UpdateBuyerInfo<'info> {
    pub fn update_buyer_info(
        &mut self,
        gender: Option<Gender>,
        profile_picture: Option<String>,
        bio: Option<String>,
        profession: Option<String>,

        interest_properties: Option<Vec<String>>,
        locations_preferred: Option<Vec<String>>,
        budgets: Option<u32>,

    ) -> Result<()> {
        require!(self.user.user_key == self.user_key.key(), EasyHavenErrors::WrongAccount);
        require!(self.user.user_type == UserType::Buyer, EasyHavenErrors::NotABuyer);
        
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

        // update buyer info
        if let Some(interests) = interest_properties {
            self.buyer.interest_properties = interests;
        }

        if let Some(loc) = locations_preferred {
            self.buyer.locations_preferred = loc;
        }

        if let Some(budget) = budgets {
            self.buyer.budgets = budget;
        }

        Ok(())
    }
}