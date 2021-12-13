use anchor_lang::prelude::*;

#[error]
pub enum ErrorCode {
    //0
    #[msg("failed to perform some math operation safely")]
    ArithmeticError,

    #[msg("unknown instruction")]
    UnknownInstruction,

    #[msg("invalid parameter")]
    InvalidParameter,

    #[msg("vault is currently locked or frozen and cannot be accessed")]
    VaultAccessSuspended,

    #[msg("recorded token amount in GDR != actual amount in gem box")]
    AmountMismatch,

    //5
    #[msg("anchor serialization issue")]
    AccountDidNotSerialize,

    #[msg("this gem is not present on any of the whitelists")]
    NotWhitelisted,

    #[msg("whitelist proof exists but for the wrong type")]
    WrongWhitelistType,
}