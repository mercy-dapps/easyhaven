use anchor_lang::prelude::*;

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
    pub user_key: Pubkey,
    pub seed: u64,
    #[max_len(50)]
    pub name: String,
    #[max_len(1000)]
    pub details: String,
    #[max_len(5, 200)]
    pub pictures: Vec<String>,
    pub num_of_owners: u8,
    #[max_len(5, u32)]
    pub list_of_owners: Vec<Pubkey>,
    pub price: u32,
    #[max_len(100)]
    pub location: String, // with google maps link
    pub property_type: PropertyType,
    pub mode_of_payment: PaymentMode,
    #[max_len(200)]
    pub terms_and_conditions: String,
    #[max_len(200)]
    pub agreement: String, // ipfs link to nft representing ownership/rental agreement
    pub approved: bool,
    pub bump: u8
}

// creation of property would be in stages
// possibly after the verification of property, then 