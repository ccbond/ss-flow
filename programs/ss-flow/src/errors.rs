use anchor_lang::error_code;

/// `ErrorCode` is the error type for the `streamflow` program.
#[error_code]
#[derive(PartialEq)]
pub enum ErrorCode {
    #[msg("Unauthorized")]
    Unauthorized,
    #[msg("Invalid interver when initializing pool")]
    InvalidInitAmount,
    #[msg("End time must be greater than start_time")]
    EndTimeMustBeGreaterThanStartTime,
    #[msg("Start time must be greater than current_time")]
    StartTimeMustBeGreaterThanCurrentTime,
    #[msg("Nft mint already initialized")]
    NftMintAlreadyInitialized,
    #[msg("Invalid nft balance")]
    InvalidNftBalance,
    #[msg("Withdraw already done")]
    WithdrawAlreadyDone,
    #[msg("Invalid withdraw time")]
    InvalidWithdrawTime,
    #[msg("Withdraw is paused")]
    WithdrawPaused,
    #[msg("Verified admin failed")]
    VerifiedAdminFailed,
    #[msg("Unlock period must be multiple of freed interval")]
    UnlockPeriodMustBeMultipleOfFreedInterval,
    #[msg("Withdraw not start")]
    WithdrawNotStart,
    #[msg("Pool withdraw already pause")]
    PoolWithdrawAlreadyPause,
    #[msg("Pool withdraw already start")]
    PoolWithdrawAlreadyStart,
    #[msg("No available withdraw token")]
    NoAvailableWithdrawToken,
    #[msg("Invaild authority nft ata")]
    InvaildAuthorityNftATA,
}
