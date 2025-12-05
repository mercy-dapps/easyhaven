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
