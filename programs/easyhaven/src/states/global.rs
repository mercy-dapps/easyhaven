use anchor_lang::prelude::*;

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


#[account]
#[derive(InitSpace)]
pub struct User {
    #[max_len(50)]
    pub name: String,
    #[max_len(50)]
    pub email: String,
    #[max_len(100)]
    pub location: String,
    #[max_len(50)]
    pub phone_number: String,
}