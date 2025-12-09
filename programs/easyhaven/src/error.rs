use anchor_lang::prelude::*;

#[error_code]
pub enum EasyHavenErrors {
    #[msg("The provided name is too long.")]
    NameTooLong,

    #[msg("You are neither a buyer/tenant")]
    NotABuyer,

    #[msg("You are not an Owner (Host)")]
    NotAOwner,

    #[msg("Wrong Account provided")]
    WrongAccount
}