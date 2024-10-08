/// Error codes returned by the bootloader.
#[derive(Debug)]
#[non_exhaustive]
pub enum BootloaderErrorCode {
    EthCall,
    AccountTxValidationFailed,
    FailedToChargeFee,
    FromIsNotAnAccount,
    FailedToCheckAccount,
    UnacceptableGasPrice,
    PayForTxFailed,
    PrePaymasterPreparationFailed,
    PaymasterValidationFailed,
    FailedToSendFeesToTheOperator,
    FailedToSetPrevBlockHash,
    UnacceptablePubdataPrice,
    TxValidationError,
    MaxPriorityFeeGreaterThanMaxFee,
    BaseFeeGreaterThanMaxFeePerGas,
    PaymasterReturnedInvalidContext,
    PaymasterContextIsTooLong,
    AssertionError,
    FailedToMarkFactoryDeps,
    TxValidationOutOfGas,
    NotEnoughGasProvided,
    AccountReturnedInvalidMagic,
    PaymasterReturnedInvalidMagic,
    MintEtherFailed,
    FailedToAppendTransactionToL2Block,
    FailedToSetL2Block,
    FailedToPublishTimestampDataToL1,
    L1MessengerPublishingFailed,
    L1MessengerLogSendingFailed,
    FailedToCallSystemContext,
    Unknown,
}

impl From<u8> for BootloaderErrorCode {
    fn from(code: u8) -> BootloaderErrorCode {
        match code {
            0 => BootloaderErrorCode::EthCall,
            1 => BootloaderErrorCode::AccountTxValidationFailed,
            2 => BootloaderErrorCode::FailedToChargeFee,
            3 => BootloaderErrorCode::FromIsNotAnAccount,
            4 => BootloaderErrorCode::FailedToCheckAccount,
            5 => BootloaderErrorCode::UnacceptableGasPrice,
            6 => BootloaderErrorCode::FailedToSetPrevBlockHash,
            7 => BootloaderErrorCode::PayForTxFailed,
            8 => BootloaderErrorCode::PrePaymasterPreparationFailed,
            9 => BootloaderErrorCode::PaymasterValidationFailed,
            10 => BootloaderErrorCode::FailedToSendFeesToTheOperator,
            11 => BootloaderErrorCode::UnacceptablePubdataPrice,
            12 => BootloaderErrorCode::TxValidationError,
            13 => BootloaderErrorCode::MaxPriorityFeeGreaterThanMaxFee,
            14 => BootloaderErrorCode::BaseFeeGreaterThanMaxFeePerGas,
            15 => BootloaderErrorCode::PaymasterReturnedInvalidContext,
            16 => BootloaderErrorCode::PaymasterContextIsTooLong,
            17 => BootloaderErrorCode::AssertionError,
            18 => BootloaderErrorCode::FailedToMarkFactoryDeps,
            19 => BootloaderErrorCode::TxValidationOutOfGas,
            20 => BootloaderErrorCode::NotEnoughGasProvided,
            21 => BootloaderErrorCode::AccountReturnedInvalidMagic,
            22 => BootloaderErrorCode::PaymasterReturnedInvalidMagic,
            23 => BootloaderErrorCode::MintEtherFailed,
            24 => BootloaderErrorCode::FailedToAppendTransactionToL2Block,
            25 => BootloaderErrorCode::FailedToSetL2Block,
            26 => BootloaderErrorCode::FailedToPublishTimestampDataToL1,
            27 => BootloaderErrorCode::L1MessengerPublishingFailed,
            28 => BootloaderErrorCode::L1MessengerLogSendingFailed,
            29 => BootloaderErrorCode::FailedToCallSystemContext,
            _ => BootloaderErrorCode::Unknown,
        }
    }
}
