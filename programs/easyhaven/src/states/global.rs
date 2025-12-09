use anchor_lang::prelude::*;

// gender enum
#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq, Default)]
pub enum Gender {
    #[default]
    UnSpecified,
    Male,
    Female,
}

impl Space for Gender {
    const INIT_SPACE: usize = 1;
}

// user type (buyer/owner)
#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq, Default)]
pub enum UserType {
    #[default]
    Buyer,
    Owner,
}

impl Space for UserType {
    const INIT_SPACE: usize = 1;
}

// review struct
#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace)]
pub struct Review {
    pub author: Pubkey,
    #[max_len(100)]
    pub text: String,
    pub timestamp: i64,
}

#[account]
#[derive(InitSpace, Default)]
pub struct User {
    pub user_key: Pubkey,

    // required to register as a user
    #[max_len(50)]
    pub name: String,
    #[max_len(50)]
    pub email: String,
    #[max_len(20)]
    pub phone_number: String,
    #[max_len(50)]
    pub location: String,

    // required to switch role
    pub user_type: UserType,

    // update user information
    pub gender: Gender,
    #[max_len(100)]
    pub profile_picture: String,
    #[max_len(500)]
    pub bio: String,
     #[max_len(50)]
    pub profession: String,

    // only buyer/tenant (points to BuyerInfo PDA)
    pub buyer_info: Option<Pubkey>,

    // only owner (points to OwnerInfo PDA)
    pub owner_info: Option<Pubkey>,

    pub bump: u8
}