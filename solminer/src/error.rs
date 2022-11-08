use anchor_lang::prelude::*;

#[error_code]
pub enum CustomError {
    #[msg("You are not authorized to perform this action.")]
    Unauthorized,

    #[msg("You are not authorized to perform this action.")]
    NotAllowedAuthority,

    #[msg("Invalid Parameter")]
    InvalidParameter,

    #[msg("Invalid Referrer")]
    InvalidReferrer,

    #[msg("You can't claim")]
    CantClaimNow,

    #[msg("You can't compound")]
    CantCompound,

    #[msg("You can't withdraw")]
    CantWithdrawNow,

    #[msg("You can't invest. Matrix count overflow")]
    MatrixCountOverflow,

    #[msg("You cant withdraw you have collected five times Already")]
    TotalRewardOverflow,

    #[msg("You already withdrew a lot than your investment")]
    WithdrawOverflow,

    #[msg("Referral can't be same as your address")]
    InvalidReferral,

    #[msg("Amount is not in the range of 1~1000 sol")]
    InvalidAmount,

    #[msg("You are blocked by Admin")]
    Blocked,

    #[msg("MinerNotStarted")]
    MinerNotStarted,

    #[msg("Miner is already started.")]
    MinerAlreadyStarted,

    #[msg("Zero address is detected.")]
    ZeroAddressDetected
}
