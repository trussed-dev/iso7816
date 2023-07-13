use crate::Data;

impl Default for Status {
    fn default() -> Self {
        Status::Success
    }
}
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum Status {
    Success,

    MoreAvailable(u8),

    DataUnchangedWarning,
    /// Triggering by the card
    ///
    /// The count must be within `0x02..=0x80`
    WarningTriggering(u8),
    CorruptedData,
    UnexpectedEof,
    SelectFileDeactivated,
    FileControlInfoBadlyFormatted,
    SelectedFileInTerminationState,
    NoInputDataFromSensor,

    /// Data changed warning
    ///
    /// Name kept for backwards compatibility
    VerificationFailed,
    FilledByLastWrite,
    /// Generic Warning Counter
    ///
    /// Meaning depends on the command
    ///
    /// The count must be within `0x00..=0x0F`
    RemainingRetries(u8),

    UnspecifiedNonpersistentExecutionError,
    ImmediateResponseRequired,
    /// Triggering by the card
    ///
    /// The count must be within `0x02..=0x80`
    ErrorTriggering(u8),

    UnspecifiedPersistentExecutionError,
    MemoryFailure,

    WrongLength,

    ClaNotSupported,
    LogicalChannelNotSupported,
    SecureMessagingNotSupported,
    LastCommandOfChainExpected,
    CommandChainingNotSupported,

    CommandNotAllowed,
    CommandIncompatibleFileStructure,
    SecurityStatusNotSatisfied,
    /// AuthenticationMethodBlocked
    ///
    /// Name kept for backwards compatiblity
    OperationBlocked,
    ReferenceDataNotUsable,
    ConditionsOfUseNotSatisfied,
    CommandNotAllowedNoEf,
    ExectedSecureMessagingDataObjectsMissing,
    IncorrectSecureMessagingDataObjects,

    WrongParametersNoInfo,
    IncorrectDataParameter,
    FunctionNotSupported,
    /// FileOrAppNotFound
    ///
    /// Name kept for backwards compatibility
    NotFound,
    RecordNotFound,
    NotEnoughMemory,
    NcInconsistentWithTlv,
    IncorrectP1OrP2Parameter,
    NcInconsistentWithP1p2,
    /// Reference not found
    ///
    /// Name kept for backwards compatibility
    KeyReferenceNotFound,
    FileAlreadyExists,
    DfNameAlreadyExists,

    WrongParameters,

    WrongLeField(u8),
    InstructionNotSupportedOrInvalid,
    ClassNotSupported,
    UnspecifiedCheckingError,

    #[doc(hidden)]
    Unknown(u16),
}

/// `0x9000`
const SUCCESS: u16 = 0x9000;

const MORE_AVAILABLE_LOWER: u16 = 0x6100;
const MORE_AVAILABLE_UPPER: u16 = 0x61FF;
const MORE_AVAILABLE_MASK: u16 = 0x00FF;

const WRONG_LE_FIELD_LOWER: u16 = 0x6C00;
const WRONG_LE_FIELD_UPPER: u16 = 0x6CFF;
const WRONG_LE_FIELD_MASK: u16 = 0x00FF;

/// `0x6200`
const DATA_UNCHANGED_WARNING: u16 = 0x6200;
const WARNING_TRIGGERING_LOWER: u16 = 0x6202;
const WARNING_TRIGGERING_MASK: u16 = 0x00FF;
const WARNING_TRIGGERING_UPPER: u16 = 0x6280;
const ERROR_TRIGGERING_LOWER: u16 = 0x6402;
const ERROR_TRIGGERING_MASK: u16 = 0x00FF;
const ERROR_TRIGGERING_UPPER: u16 = 0x6480;
/// `0x6281`
const CORRUPTED_DATA: u16 = 0x6281;
/// `0x6282`
const UNEXPECTED_EOF: u16 = 0x6282;
/// `0x6283`
const SELECT_FILE_DEACTIVATED: u16 = 0x6283;
/// `0x6284`
const FILE_CONTROL_INFO_BADLY_FORMATTED: u16 = 0x6284;
/// `0x6285`
const SELECTED_FILE_IN_TERMINATION_STATE: u16 = 0x6285;
/// `0x6286`
const NO_INPUT_DATA_FROM_SENSOR: u16 = 0x6286;

/// 0x6400
const EXECUTION_ERROR: u16 = 0x6400;
/// 0x6401
const IMMEDIATE_RESPONSE_REQUIRED: u16 = 0x6401;

/// `0x6300`
const DATA_CHANGED_WARNING: u16 = 0x6300;
/// `0x6381`
const FILLED_BY_LAST_WRITE: u16 = 0x6381;
const WARNING_COUNTER_LOWER: u16 = 0x63C0;
const WARNING_COUNTER_UPPER: u16 = 0x63CF;
const WARNING_COUNTER_MASK: u16 = 0x000F;

/// `0x6500`
const DATA_CHANGED_ERROR: u16 = 0x6500;
/// `0x6581`
const MEMORY_FAILURE: u16 = 0x6581;

/// `0x6700`
const WRONG_LENGTH: u16 = 0x6700;

/// `0x6800`
const CLA_NOT_SUPPORTED: u16 = 0x6800;
/// `0x6881`
const LOGICAL_CHANNEL_NOT_SUPPORTED: u16 = 0x6881;
/// `0x6882`
const SECURE_MESSAGING_NOT_SUPPORTED: u16 = 0x6882;
/// `0x6883`
const LAST_COMMAND_OF_CHAIN_EXPECTED: u16 = 0x6883;
/// `0x6884`
const COMMAND_CHAINING_NOT_SUPPORTED: u16 = 0x6884;

/// `0x6900`
const COMMAND_NOT_ALLOWED: u16 = 0x6900;
/// `0x6981`
const COMMAND_INCOMPATIBLE_FILE_STRUCTURE: u16 = 0x6981;
/// `0x6982`
const SECURITY_STATUS_NOT_SATISFIED: u16 = 0x6982;
/// `0x6983`
const AUTHENTICATION_METHOD_BLOCKED: u16 = 0x6983;
/// `0x6984`
const REFERENCE_DATA_NOT_USABLE: u16 = 0x6984;
/// `0x6985`
const CONDITIONS_OF_USE_NOT_SATISFIED: u16 = 0x6985;
/// `0x6986`
const COMMAND_NOT_ALLOWED_NO_EF: u16 = 0x6986;
/// `0x6987`
const EXECTED_SECURE_MESSAGING_DATA_OBJECTS_MISSING: u16 = 0x6987;
/// `0x6988`
const INCORRECT_SECURE_MESSAGING_DATA_OBJECTS: u16 = 0x6988;

/// `0x6A00`
const WRONG_PARAMETERS_NO_INFO: u16 = 0x6A00;
/// `0x6A80`
const INCORRECT_PARAMETERS: u16 = 0x6A80;
/// `0x6A81`
const FUNCTION_NOT_SUPPORTED: u16 = 0x6A81;
/// `0x6A82`
const FILE_OR_APP_NOT_FOUND: u16 = 0x6A82;
/// `0x6A83`
const RECORD_NOT_FOUND: u16 = 0x6A83;
/// `0x6A84`
const NOT_ENOUGH_MEMORY_IN_FILE: u16 = 0x6A84;
/// `0x6A85`
const NC_INCONSISTENT_WITH_TLV: u16 = 0x6A85;
/// `0x6A86`
const INCORRECT_P1P2: u16 = 0x6A86;
/// `0x6A87`
const NC_INCONSISTENT_WITH_P1P2: u16 = 0x6A87;
/// `0x6A88`
const REFERENCE_NOT_FOUND: u16 = 0x6A88;
/// `0x6A89`
const FILE_ALREADY_EXISTS: u16 = 0x6A89;
/// `0x6A8A`
const DF_NAME_ALREADY_EXISTS: u16 = 0x6A8A;

/// `0x6B00`
const WRONG_PARAMETERS: u16 = 0x6B00;

/// `0x6D00`
const INSTRUCTION_NOT_SUPPORTED_OR_INVALID: u16 = 0x6D00;
/// `0x6E00`
const CLASS_NOT_SUPPORTED: u16 = 0x6E00;
/// `0x6F00`
const CHECKING_ERROR: u16 = 0x6F00;

impl Status {
    pub const fn from_u16(sw: u16) -> Self {
        match sw {
            SUCCESS => Status::Success,

            DATA_UNCHANGED_WARNING => Status::DataUnchangedWarning,
            CORRUPTED_DATA => Status::CorruptedData,
            UNEXPECTED_EOF => Status::UnexpectedEof,
            SELECT_FILE_DEACTIVATED => Status::SelectFileDeactivated,
            FILE_CONTROL_INFO_BADLY_FORMATTED => Status::FileControlInfoBadlyFormatted,
            SELECTED_FILE_IN_TERMINATION_STATE => Status::SelectedFileInTerminationState,
            NO_INPUT_DATA_FROM_SENSOR => Status::NoInputDataFromSensor,

            DATA_CHANGED_WARNING => Status::VerificationFailed,
            FILLED_BY_LAST_WRITE => Status::FilledByLastWrite,

            EXECUTION_ERROR => Status::UnspecifiedNonpersistentExecutionError,
            IMMEDIATE_RESPONSE_REQUIRED => Status::ImmediateResponseRequired,

            DATA_CHANGED_ERROR => Status::UnspecifiedPersistentExecutionError,
            MEMORY_FAILURE => Status::MemoryFailure,

            WRONG_LENGTH => Status::WrongLength,

            CLA_NOT_SUPPORTED => Status::ClaNotSupported,
            LOGICAL_CHANNEL_NOT_SUPPORTED => Status::LogicalChannelNotSupported,
            SECURE_MESSAGING_NOT_SUPPORTED => Status::SecureMessagingNotSupported,
            LAST_COMMAND_OF_CHAIN_EXPECTED => Status::LastCommandOfChainExpected,
            COMMAND_CHAINING_NOT_SUPPORTED => Status::CommandChainingNotSupported,

            COMMAND_NOT_ALLOWED => Status::CommandNotAllowed,
            COMMAND_INCOMPATIBLE_FILE_STRUCTURE => Status::CommandIncompatibleFileStructure,
            SECURITY_STATUS_NOT_SATISFIED => Status::SecurityStatusNotSatisfied,
            AUTHENTICATION_METHOD_BLOCKED => Status::OperationBlocked,
            REFERENCE_DATA_NOT_USABLE => Status::ReferenceDataNotUsable,
            CONDITIONS_OF_USE_NOT_SATISFIED => Status::ConditionsOfUseNotSatisfied,
            COMMAND_NOT_ALLOWED_NO_EF => Status::CommandNotAllowedNoEf,
            EXECTED_SECURE_MESSAGING_DATA_OBJECTS_MISSING => {
                Status::ExectedSecureMessagingDataObjectsMissing
            }
            INCORRECT_SECURE_MESSAGING_DATA_OBJECTS => Status::IncorrectSecureMessagingDataObjects,

            WRONG_PARAMETERS_NO_INFO => Status::WrongParametersNoInfo,
            INCORRECT_PARAMETERS => Status::IncorrectDataParameter,
            FUNCTION_NOT_SUPPORTED => Status::FunctionNotSupported,
            FILE_OR_APP_NOT_FOUND => Status::NotFound,
            RECORD_NOT_FOUND => Status::RecordNotFound,
            NOT_ENOUGH_MEMORY_IN_FILE => Status::NotEnoughMemory,
            NC_INCONSISTENT_WITH_TLV => Status::NcInconsistentWithTlv,
            INCORRECT_P1P2 => Status::IncorrectP1OrP2Parameter,
            NC_INCONSISTENT_WITH_P1P2 => Status::NcInconsistentWithP1p2,
            REFERENCE_NOT_FOUND => Status::KeyReferenceNotFound,
            FILE_ALREADY_EXISTS => Status::FileAlreadyExists,
            DF_NAME_ALREADY_EXISTS => Status::DfNameAlreadyExists,

            WRONG_PARAMETERS => Status::WrongParameters,

            INSTRUCTION_NOT_SUPPORTED_OR_INVALID => Status::InstructionNotSupportedOrInvalid,
            CLASS_NOT_SUPPORTED => Status::ClassNotSupported,
            CHECKING_ERROR => Status::UnspecifiedCheckingError,
            v @ WARNING_TRIGGERING_LOWER..=WARNING_TRIGGERING_UPPER => {
                Self::WarningTriggering((v & WARNING_TRIGGERING_MASK) as u8)
            }
            v @ ERROR_TRIGGERING_LOWER..=ERROR_TRIGGERING_UPPER => {
                Self::ErrorTriggering((v & ERROR_TRIGGERING_MASK) as u8)
            }
            v @ MORE_AVAILABLE_LOWER..=MORE_AVAILABLE_UPPER => {
                Self::MoreAvailable((v & MORE_AVAILABLE_MASK) as u8)
            }
            v @ WRONG_LE_FIELD_LOWER..=WRONG_LE_FIELD_UPPER => {
                Self::WrongLeField((v & WRONG_LE_FIELD_MASK) as u8)
            }
            v @ WARNING_COUNTER_LOWER..=WARNING_COUNTER_UPPER => {
                Self::RemainingRetries((v & WARNING_COUNTER_MASK) as u8)
            }
            v @ _ => Self::Unknown(v),
        }
    }
}

impl From<u16> for Status {
    #[inline]
    fn from(sw: u16) -> Self {
        Self::from_u16(sw)
    }
}

impl From<(u8, u8)> for Status {
    fn from((sw1, sw2): (u8, u8)) -> Self {
        [sw1, sw2].into()
    }
}

impl From<[u8; 2]> for Status {
    fn from(sw: [u8; 2]) -> Self {
        u16::from_be_bytes(sw).into()
    }
}

impl From<Status> for u16 {
    #[inline]
    fn from(status: Status) -> u16 {
        match status {
            Status::Success => SUCCESS,

            Status::DataUnchangedWarning => DATA_UNCHANGED_WARNING,
            Status::CorruptedData => CORRUPTED_DATA,
            Status::UnexpectedEof => UNEXPECTED_EOF,
            Status::SelectFileDeactivated => SELECT_FILE_DEACTIVATED,
            Status::FileControlInfoBadlyFormatted => FILE_CONTROL_INFO_BADLY_FORMATTED,
            Status::SelectedFileInTerminationState => SELECTED_FILE_IN_TERMINATION_STATE,
            Status::NoInputDataFromSensor => NO_INPUT_DATA_FROM_SENSOR,

            Status::VerificationFailed => DATA_CHANGED_WARNING,
            Status::FilledByLastWrite => FILLED_BY_LAST_WRITE,

            Status::UnspecifiedNonpersistentExecutionError => EXECUTION_ERROR,
            Status::ImmediateResponseRequired => IMMEDIATE_RESPONSE_REQUIRED,

            Status::UnspecifiedPersistentExecutionError => DATA_CHANGED_ERROR,
            Status::MemoryFailure => MEMORY_FAILURE,

            Status::WrongLength => WRONG_LENGTH,

            Status::ClaNotSupported => CLA_NOT_SUPPORTED,
            Status::LogicalChannelNotSupported => LOGICAL_CHANNEL_NOT_SUPPORTED,
            Status::SecureMessagingNotSupported => SECURE_MESSAGING_NOT_SUPPORTED,
            Status::LastCommandOfChainExpected => LAST_COMMAND_OF_CHAIN_EXPECTED,
            Status::CommandChainingNotSupported => COMMAND_CHAINING_NOT_SUPPORTED,

            Status::CommandNotAllowed => COMMAND_NOT_ALLOWED,
            Status::CommandIncompatibleFileStructure => COMMAND_INCOMPATIBLE_FILE_STRUCTURE,
            Status::SecurityStatusNotSatisfied => SECURITY_STATUS_NOT_SATISFIED,
            Status::OperationBlocked => AUTHENTICATION_METHOD_BLOCKED,
            Status::ReferenceDataNotUsable => REFERENCE_DATA_NOT_USABLE,
            Status::ConditionsOfUseNotSatisfied => CONDITIONS_OF_USE_NOT_SATISFIED,
            Status::CommandNotAllowedNoEf => COMMAND_NOT_ALLOWED_NO_EF,
            Status::ExectedSecureMessagingDataObjectsMissing => {
                EXECTED_SECURE_MESSAGING_DATA_OBJECTS_MISSING
            }
            Status::IncorrectSecureMessagingDataObjects => INCORRECT_SECURE_MESSAGING_DATA_OBJECTS,

            Status::WrongParametersNoInfo => WRONG_PARAMETERS_NO_INFO,
            Status::IncorrectDataParameter => INCORRECT_PARAMETERS,
            Status::FunctionNotSupported => FUNCTION_NOT_SUPPORTED,
            Status::NotFound => FILE_OR_APP_NOT_FOUND,
            Status::RecordNotFound => RECORD_NOT_FOUND,
            Status::NotEnoughMemory => NOT_ENOUGH_MEMORY_IN_FILE,
            Status::NcInconsistentWithTlv => NC_INCONSISTENT_WITH_TLV,
            Status::IncorrectP1OrP2Parameter => INCORRECT_P1P2,
            Status::NcInconsistentWithP1p2 => NC_INCONSISTENT_WITH_P1P2,
            Status::KeyReferenceNotFound => REFERENCE_NOT_FOUND,
            Status::FileAlreadyExists => FILE_ALREADY_EXISTS,
            Status::DfNameAlreadyExists => DF_NAME_ALREADY_EXISTS,

            Status::WrongParameters => WRONG_PARAMETERS,

            Status::InstructionNotSupportedOrInvalid => INSTRUCTION_NOT_SUPPORTED_OR_INVALID,
            Status::ClassNotSupported => CLASS_NOT_SUPPORTED,
            Status::UnspecifiedCheckingError => CHECKING_ERROR,
            Status::WarningTriggering(v) => WARNING_TRIGGERING_LOWER + v as u16,
            Status::ErrorTriggering(v) => ERROR_TRIGGERING_LOWER + v as u16,
            Status::MoreAvailable(v) => MORE_AVAILABLE_LOWER + v as u16,
            Status::WrongLeField(v) => WRONG_LE_FIELD_LOWER + v as u16,
            Status::RemainingRetries(v) => WARNING_COUNTER_LOWER + v as u16,
            Status::Unknown(v) => v,
        }
    }
}

impl From<Status> for [u8; 2] {
    #[inline]
    fn from(status: Status) -> [u8; 2] {
        let sw: u16 = status.into();
        sw.to_be_bytes()
    }
}

impl<const S: usize> From<Status> for Data<S> {
    #[inline]
    fn from(status: Status) -> Data<S> {
        let arr: [u8; 2] = status.into();
        Data::from_slice(&arr).unwrap()
    }
}
