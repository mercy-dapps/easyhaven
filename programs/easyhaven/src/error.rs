use anchor_lang::prelude::*;

#[error_code]
pub enum EasyHavenErrors {
    #[msg("The provided name is too long.")]
    NameTooLong
}