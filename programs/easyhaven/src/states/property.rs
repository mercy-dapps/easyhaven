use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum PropertyType {
    Rental,
    Ownership
}

impl Space for PropertyType {
    const INIT_SPACE: usize = 1;
}


#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum PaymentMode {
    Fiat,
    Crypto
}

impl Space for PaymentMode {
    const INIT_SPACE: usize = 1;
}

#[account]
#[derive(InitSpace)]

pub struct Property {
    pub owner: Pubkey,
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
    pub bump: u8
}