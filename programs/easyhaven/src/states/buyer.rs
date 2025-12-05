use anchor_lang::prelude::*;

use crate::global::Gender;

#[account]
#[derive(InitSpace)]
pub struct Buyer {
    pub buyer: Pubkey,
    #[max_len(50)]
    pub name: String,
    #[max_len(200)]
    pub profile_picture: String,
    #[max_len(1000)]
    pub bio: String,
    #[max_len(50)]
    pub email: String,
    pub gender: Gender,
    #[max_len(50)]
    pub phone_number: String,
     #[max_len(50)]
    pub profession: String,
    #[max_len(3, 100)]
    pub interest_properties: Vec<String>,
    #[max_len(100)]
    pub location: String,
    #[max_len(5, 200)]
    pub locations_preferred: Vec<String>,
    pub budgets: u8,
    pub bump: u8
}