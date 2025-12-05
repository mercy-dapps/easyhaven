use anchor_lang::prelude::*;

use crate::global::Gender;

#[account]
#[derive(InitSpace, Default)]
pub struct PropertyOwner {
    pub owner: Pubkey,
    #[max_len(50)]
    pub name: String,
    #[max_len(50)]
    pub email: String,
    pub gender: Gender,
    #[max_len(50)]
    pub phone_number: String,
     #[max_len(50)]
    pub profession: String,
    #[max_len(200)]
    pub profile_picture: String,
    #[max_len(1000)]
    pub bio: String,
    pub reviews: u32,
    pub ratings: f32,
    #[max_len(100)]
    pub location: String,
    pub listings: u8,
    #[max_len(5, 50)]
    pub languages_spoken: Vec<String>,
    pub bump: u8
}