use anchor_lang::prelude::*;

use crate::Review;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq, Default)]
pub enum PropertyType {
    Rental,
    #[default]
    Ownership
}

impl Space for PropertyType {
    const INIT_SPACE: usize = 1;
}


#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq, Default)]
pub enum PaymentMode {
    Fiat,
    #[default]
    Crypto
}

impl Space for PaymentMode {
    const INIT_SPACE: usize = 1;
}

#[account]
#[derive(InitSpace, Default)]

pub struct Property {
    pub user_key: Pubkey, // owner of the property
    pub seed: u64,
    #[max_len(50)]
    pub name: String,
    #[max_len(1000)]
    pub details: String,
    #[max_len(5, 100)]
    pub pictures: Vec<String>,
    pub price: u32,
    #[max_len(100)]
    pub location: String, // with google maps link
    pub property_type: PropertyType,
    pub mode_of_payment: PaymentMode,
    #[max_len(100)]
    pub terms_and_conditions: String,
    #[max_len(100)]
    pub agreement: String, // ipfs link to nft representing ownership/rental agreement

    pub approved: bool,

    #[max_len(5)]
    pub list_of_owners: Vec<Pubkey>,
    #[max_len(10)]
    pub liked_pubkey: Vec<Pubkey>,
     #[max_len(10)]
    pub saved_pubkey: Vec<Pubkey>,
    #[max_len(10)]
    pub rate: Vec<u8>,
    #[max_len(10)]
    pub reviews: Vec<Review>,
    
    pub bump: u8
}

// creation of property would be in stages
// possibly after the verification of property