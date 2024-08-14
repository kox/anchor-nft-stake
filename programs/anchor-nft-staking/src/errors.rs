use anchor_lang::prelude::*;

#[error_codes]
pub enum ErrorCode {
    #[msg!("maxium amount stake exceeded")]
    ExceedMaxStake
}