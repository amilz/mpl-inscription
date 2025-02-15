//! This code was AUTOGENERATED using the kinobi library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun kinobi to update it.
//!
//! [https://github.com/metaplex-foundation/kinobi]
//!

use num_derive::FromPrimitive;
use thiserror::Error;

#[derive(Clone, Debug, Eq, Error, FromPrimitive, PartialEq)]
pub enum MplInscriptionError {
    /// 0 (0x0) - The account has already been initialized
    #[error("The account has already been initialized")]
    AlreadyInitialized,
    /// 1 (0x1) - The account has not yet been initialized
    #[error("The account has not yet been initialized")]
    NotInitialized,
    /// 2 (0x2) - The key for the account is invalid.
    #[error("The key for the account is invalid.")]
    DerivedKeyInvalid,
    /// 3 (0x3) - The system program account is invalid.
    #[error("The system program account is invalid.")]
    InvalidSystemProgram,
    /// 4 (0x4) - The JSON data is invalid.
    #[error("The JSON data is invalid.")]
    InvalidJson,
    /// 5 (0x5) - Borsh failed to serialize this account.
    #[error("Borsh failed to serialize this account.")]
    BorshSerializeError,
    /// 6 (0x6) - Borsh failed to deserialize this account.
    #[error("Borsh failed to deserialize this account.")]
    BorshDeserializeError,
    /// 7 (0x7) - The payer does not have authority to perform this action.
    #[error("The payer does not have authority to perform this action.")]
    InvalidAuthority,
    /// 8 (0x8) - Numerical Overflow
    #[error("Numerical Overflow")]
    NumericalOverflow,
    /// 9 (0x9) - Incorrect Owner
    #[error("Incorrect Owner")]
    IncorrectOwner,
    /// 10 (0xA) - Mint Mismatch between Metadata and Mint Accounts.
    #[error("Mint Mismatch between Metadata and Mint Accounts.")]
    MintMismatch,
    /// 11 (0xB) - Must be a NonFungible Token
    #[error("Must be a NonFungible Token")]
    InvalidTokenStandard,
    /// 12 (0xC) - Not enough tokens in the provided token account.
    #[error("Not enough tokens in the provided token account.")]
    NotEnoughTokens,
    /// 13 (0xD) - The shard account is invalid.
    #[error("The shard account is invalid.")]
    InvalidShardAccount,
    /// 14 (0xE) - The association tag cannot be blank.
    #[error("The association tag cannot be blank.")]
    AssociationTagCannotBeBlank,
    /// 15 (0xF) - The association tag is too long.
    #[error("The association tag is too long.")]
    AssociationTagTooLong,
    /// 16 (0x10) - The authority already exists.
    #[error("The authority already exists.")]
    AuthorityAlreadyExists,
}

impl solana_program::program_error::PrintProgramError for MplInscriptionError {
    fn print<E>(&self) {
        solana_program::msg!(&self.to_string());
    }
}
