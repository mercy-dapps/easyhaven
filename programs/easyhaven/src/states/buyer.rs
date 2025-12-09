use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace, Default)]
pub struct BuyerInfo {
    pub user: Pubkey,
     #[max_len(3, 50)]
    pub interest_properties: Vec<String>,
    #[max_len(5, 50)]
    pub locations_preferred: Vec<String>,
    pub budgets: u32,
    pub bump: u8
}