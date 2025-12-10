use anchor_lang::prelude::*;

use crate::{Review};

#[account]
#[derive(InitSpace, Default)]
pub struct OwnerInfo {
    pub user_key: Pubkey,
    pub listings: u8,
    #[max_len(2, 30)]
    pub languages_spoken: Vec<String>,
    #[max_len(50)]
    pub ratings: Vec<f32>,
    #[max_len(10)]
    pub reviews: Vec<Review>,
    pub bump: u8
}