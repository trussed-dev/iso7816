use core::fmt::{Debug, Display};

#[derive(Copy, Clone, Eq, PartialEq, PartialOrd, Ord, Debug)]
#[non_exhaustive]
pub enum StatusKind {
    Success,

    MoreAvailable(u8),

    DataUnchangedWarning,
    /// Triggering by the card
    WarningTriggering(u8),
    CorruptedData,
    UnexpectedEof,
    SelectFileDeactivated,
    FileControlInfoBadlyFormatted,
    SelectFileInTerminationState,
    NoInputDataFromSensor,

    DataChangedWarning,
    FilledByLastWrite,
    /// Meaning depends on the command
    WarningCounter(u8),

    ExecutionError,
    ImmediateResponseRequired,
    /// Triggering by the card
    ErrorTriggering(u8),

    DataChangedError,
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
    AuthenticationMethodBlocked,
    ReferenceDataNotUsable,
    ConditionOfUseNotSatisfied,
    CommandNotAllowedNoEf,
    ExectedSecureMessagingDataObjectsMissing,
    IncorrectSecureMessagingDataObjects,

    WrongParametersNoInfo,
    IncorrectParameters,
    FunctionNotSupported,
    FileOrAppNotFound,
    RecordNotFound,
    NotEnoughMemoryInFile,
    NcInconsistentWithTlv,
    IncorrectP1p2,
    NcInconsistentWithP1p2,
    ReferenceNotFound,
    FileAlreadyExists,
    DfNameAlreadyExists,

    WrongParameters,

    WrongLeField(u8),
    InstructionNotSupportedOrInvalid,
    ClassNotSupported,
    Error,
}

/// Status bytes from a response APDU.
///
/// This structure can represent any status bytes from a response APDU. For convinience, constants are provided for pattern matching.
///
/// The [`kind`](Status::kind) method can be used to obtain an Enum that can be used to make matching more convenient but is not exhaustive.
#[derive(Copy, Clone, Eq, PartialEq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct Status(pub u16);

impl Default for Status {
    fn default() -> Self {
        Self::SUCCESS
    }
}

impl Debug for Status {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self.kind() {
            Some(k) => write!(f, "{k:?} ({:02x})", self.0),
            None => write!(f, "Unknown {:02x}", self.0),
        }
    }
}

impl Status {
    /// `0x9000`
    pub const SUCCESS: Self = Self(0x9000);

    const MORE_AVAILABLE_MASK: u16 = 0x6100;

    /// `0x6200`
    pub const DATA_UNCHANGED_WARNING: Self = Self(0x6200);
    const WARNING_TRIGGERING_LOWER: u16 = 0x6202;
    const WARNING_TRIGGERING_MASK: u16 = 0x6200;
    const WARNING_TRIGGERING_UPPER: u16 = 0x6280;
    const ERROR_TRIGGERING_LOWER: u16 = 0x6402;
    const ERROR_TRIGGERING_MASK: u16 = 0x6400;
    const ERROR_TRIGGERING_UPPER: u16 = 0x6480;
    /// `0x6281`
    pub const CORRUPTED_DATA: Self = Self(0x6281);
    /// `0x6282`
    pub const UNEXPECTED_EOF: Self = Self(0x6282);
    /// `0x6283`
    pub const SELECT_FILE_DEACTIVATED: Self = Self(0x6283);
    /// `0x6284`
    pub const FILE_CONTROL_INFO_BADLY_FORMATTED: Self = Self(0x6284);
    /// `0x6285`
    pub const SELECT_FILE_IN_TERMINATION_STATE: Self = Self(0x6285);
    /// `0x6286`
    pub const NO_INPUT_DATA_FROM_SENSOR: Self = Self(0x6286);

    /// 0x6400
    pub const EXECUTION_ERROR: Self = Self(0x6400);
    /// 0x6401
    pub const IMMEDIATE_RESPONSE_REQUIRED: Self = Self(0x6401);

    /// `0x6300`
    pub const DATA_CHANGED_WARNING: Self = Self(0x6300);
    /// `0x6381`
    pub const FILLED_BY_LAST_WRITE: Self = Self(0x6381);
    const WARNING_COUNTER_MASK: u16 = 0x63C0;

    /// `0x6500`
    pub const DATA_CHANGED_ERROR: Self = Self(0x6500);
    /// `0x6581`
    pub const MEMORY_FAILURE: Self = Self(0x6581);

    /// `0x6700`
    pub const WRONG_LENGTH: Self = Self(0x6700);

    /// `0x6800`
    pub const CLA_NOT_SUPPORTED: Self = Self(0x6800);
    /// `0x6881`
    pub const LOGICAL_CHANNEL_NOT_SUPPORTED: Self = Self(0x6881);
    /// `0x6882`
    pub const SECURE_MESSAGING_NOT_SUPPORTED: Self = Self(0x6882);
    /// `0x6883`
    pub const LAST_COMMAND_OF_CHAIN_EXPECTED: Self = Self(0x6883);
    /// `0x6884`
    pub const COMMAND_CHAINING_NOT_SUPPORTED: Self = Self(0x6884);

    /// `0x6900`
    pub const COMMAND_NOT_ALLOWED: Self = Self(0x6900);
    /// `0x6981`
    pub const COMMAND_INCOMPATIBLE_FILE_STRUCTURE: Self = Self(0x6981);
    /// `0x6982`
    pub const SECURITY_STATUS_NOT_SATISFIED: Self = Self(0x6982);
    /// `0x6983`
    pub const AUTHENTICATION_METHOD_BLOCKED: Self = Self(0x6983);
    /// `0x6984`
    pub const REFERENCE_DATA_NOT_USABLE: Self = Self(0x6984);
    /// `0x6985`
    pub const CONDITION_OF_USE_NOT_SATISFIED: Self = Self(0x6985);
    /// `0x6986`
    pub const COMMAND_NOT_ALLOWED_NO_EF: Self = Self(0x6986);
    /// `0x6987`
    pub const EXECTED_SECURE_MESSAGING_DATA_OBJECTS_MISSING: Self = Self(0x6987);
    /// `0x6988`
    pub const INCORRECT_SECURE_MESSAGING_DATA_OBJECTS: Self = Self(0x6988);

    /// `0x6A00`
    pub const WRONG_PARAMETERS_NO_INFO: Self = Self(0x6A00);
    /// `0x6A80`
    pub const INCORRECT_PARAMETERS: Self = Self(0x6A80);
    /// `0x6A81`
    pub const FUNCTION_NOT_SUPPORTED: Self = Self(0x6A81);
    /// `0x6A82`
    pub const FILE_OR_APP_NOT_FOUND: Self = Self(0x6A82);
    /// `0x6A83`
    pub const RECORD_NOT_FOUND: Self = Self(0x6A83);
    /// `0x6A84`
    pub const NOT_ENOUGH_MEMORY_IN_FILE: Self = Self(0x6A84);
    /// `0x6A85`
    pub const NC_INCONSISTENT_WITH_TLV: Self = Self(0x6A85);
    /// `0x6A86`
    pub const INCORRECT_P1P2: Self = Self(0x6A86);
    /// `0x6A87`
    pub const NC_INCONSISTENT_WITH_P1P2: Self = Self(0x6A87);
    /// `0x6A88`
    pub const REFERENCE_NOT_FOUND: Self = Self(0x6A88);
    /// `0x6A89`
    pub const FILE_ALREADY_EXISTS: Self = Self(0x6A89);
    /// `0x6A8A`
    pub const DF_NAME_ALREADY_EXISTS: Self = Self(0x6A8A);

    /// `0x6B00`
    pub const WRONG_PARAMETERS: Self = Self(0x6B00);

    const WRONG_LE_FIELD_MASK: u16 = 0x6C00;

    /// `0x6D00`
    pub const INSTRUCTION_NOT_SUPPORTED_OR_INVALID: Self = Self(0x6D00);
    /// `0x6E00`
    pub const CLASS_NOT_SUPPORTED: Self = Self(0x6E00);
    /// `0x6F00`
    pub const ERROR: Self = Self(0x6F00);

    /// Create a status representing a wrong LE field (`0x6CXX`)
    pub const fn wrong_le_field(available_bytes: u8) -> Self {
        Self(Self::WRONG_LE_FIELD_MASK | available_bytes as u16)
    }

    /// Create a status indicating that more data is available (`0x61XX`)
    pub const fn more_available(value: u8) -> Self {
        Self(Self::MORE_AVAILABLE_MASK | value as u16)
    }

    pub const fn as_more_available(self) -> Option<u8> {
        if self.0 & 0xFF00 == Self::MORE_AVAILABLE_MASK {
            Some((self.0 & 0x00FF) as u8)
        } else {
            None
        }
    }
    pub const fn is_more_available(self) -> bool {
        self.as_more_available().is_some()
    }

    pub const fn is_warning(self) -> bool {
        self.is_warning_without_modification() || self.is_warning_with_modification()
    }

    /// The proccessing raised a warning and did not change state
    pub const fn is_warning_without_modification(self) -> bool {
        (self.0 & 0xFF00) == 0x6200
    }
    /// The proccessing raised a warning and changed state
    pub const fn is_warning_with_modification(self) -> bool {
        (self.0 & 0xFF00) == 0x6300
    }

    pub const fn is_execution_error(self) -> bool {
        self.0 >= 0x6400 && self.0 <= 0x6600
    }
    pub const fn is_checking_error(self) -> bool {
        self.0 >= 0x6700 && self.0 <= 0x6F00
    }

    pub const fn is_error(self) -> bool {
        self.is_execution_error() || self.is_checking_error()
    }

    pub const fn is_warning_triggering(self) -> bool {
        self.as_warning_triggering().is_some()
    }
    pub const fn as_warning_triggering(self) -> Option<u8> {
        if matches!(
            self.0,
            Self::WARNING_TRIGGERING_LOWER..=Self::WARNING_TRIGGERING_UPPER
        ) {
            Some((self.0 & 0x00FF) as u8)
        } else {
            None
        }
    }

    /// Value must be `0x02 <= value < 0x81`, otherwise panics
    pub const fn warning_triggering(value: u8) -> Self {
        match Self::try_warning_triggering(value) {
            Ok(s) => s,
            Err(_) => panic!("Expected 0x02 <= value < 0x81"),
        }
    }
    /// Value must be `0x02 <= value < 0x81`, otherwise errors
    pub const fn try_warning_triggering(value: u8) -> Result<Self, TriggeringError> {
        if value <= 0x80 && value >= 0x02 {
            Ok(Self(Self::WARNING_TRIGGERING_MASK | value as u16))
        } else {
            Err(TriggeringError)
        }
    }

    pub const fn is_warning_counter(self) -> bool {
        self.as_warning_counter().is_some()
    }

    pub const fn as_warning_counter(self) -> Option<u8> {
        if self.0 & Self::WARNING_COUNTER_MASK == Self::WARNING_COUNTER_MASK {
            Some((self.0 & 0x00F) as u8)
        } else {
            None
        }
    }

    /// Create a warning counter status. (Meaning depends on the command)
    ///
    /// Value must be `0x00 <= value < 0x0F`, otherwise errors
    pub const fn try_warning_counter(value: u8) -> Result<Self, WarningCounterError> {
        if value <= 0xF {
            Ok(Self(Self::WARNING_COUNTER_MASK | value as u16))
        } else {
            Err(WarningCounterError)
        }
    }
    /// Value must be `0x00 <= value < 0x0F`, otherwise panics
    pub const fn warning_counter(value: u8) -> Self {
        match Self::try_warning_counter(value) {
            Ok(s) => s,
            Err(_) => panic!("Expected 0x00 <= value < 0x0F"),
        }
    }
    pub const fn is_error_triggering(self) -> bool {
        self.as_error_triggering().is_some()
    }
    pub const fn as_error_triggering(self) -> Option<u8> {
        if matches!(
            self.0,
            Self::ERROR_TRIGGERING_LOWER..=Self::ERROR_TRIGGERING_UPPER
        ) {
            Some((self.0 & 0x00FF) as u8)
        } else {
            None
        }
    }

    /// Value must be `0x02 <= value < 0x81`, otherwise panics
    pub const fn error_triggering(value: u8) -> Self {
        match Self::try_error_triggering(value) {
            Ok(s) => s,
            Err(_) => panic!("Expected 0x02 <= value < 0x81"),
        }
    }
    /// Value must be `0x02 <= value < 0x81`, otherwise errors
    pub const fn try_error_triggering(value: u8) -> Result<Self, TriggeringError> {
        if value <= 0x80 && value >= 0x02 {
            Ok(Self(Self::ERROR_TRIGGERING_MASK | value as u16))
        } else {
            Err(TriggeringError)
        }
    }

    pub const fn as_wrong_le_field(self) -> Option<u8> {
        if self.0 & Self::WRONG_LE_FIELD_MASK == Self::WRONG_LE_FIELD_MASK {
            Some((self.0 & 0x00FF) as u8)
        } else {
            None
        }
    }
    pub const fn is_wrong_le_field(self) -> bool {
        self.as_wrong_le_field().is_some()
    }
    pub const fn as_bytes(self) -> [u8; 2] {
        self.0.to_be_bytes()
    }

    pub const fn as_u16(self) -> u16 {
        self.0
    }

    pub fn kind(self) -> Option<StatusKind> {
        Some(match self {
            Self::SUCCESS => StatusKind::Success,

            Self::DATA_UNCHANGED_WARNING => StatusKind::DataUnchangedWarning,
            Self::CORRUPTED_DATA => StatusKind::CorruptedData,
            Self::UNEXPECTED_EOF => StatusKind::UnexpectedEof,
            Self::SELECT_FILE_DEACTIVATED => StatusKind::SelectFileDeactivated,
            Self::FILE_CONTROL_INFO_BADLY_FORMATTED => StatusKind::FileControlInfoBadlyFormatted,
            Self::SELECT_FILE_IN_TERMINATION_STATE => StatusKind::SelectFileInTerminationState,
            Self::NO_INPUT_DATA_FROM_SENSOR => StatusKind::NoInputDataFromSensor,

            Self::DATA_CHANGED_WARNING => StatusKind::DataChangedWarning,
            Self::FILLED_BY_LAST_WRITE => StatusKind::FilledByLastWrite,

            Self::EXECUTION_ERROR => StatusKind::ExecutionError,
            Self::IMMEDIATE_RESPONSE_REQUIRED => StatusKind::ImmediateResponseRequired,

            Self::DATA_CHANGED_ERROR => StatusKind::DataChangedError,
            Self::MEMORY_FAILURE => StatusKind::MemoryFailure,

            Self::WRONG_LENGTH => StatusKind::WrongLength,

            Self::CLA_NOT_SUPPORTED => StatusKind::ClaNotSupported,
            Self::LOGICAL_CHANNEL_NOT_SUPPORTED => StatusKind::LogicalChannelNotSupported,
            Self::SECURE_MESSAGING_NOT_SUPPORTED => StatusKind::SecureMessagingNotSupported,
            Self::LAST_COMMAND_OF_CHAIN_EXPECTED => StatusKind::LastCommandOfChainExpected,
            Self::COMMAND_CHAINING_NOT_SUPPORTED => StatusKind::CommandChainingNotSupported,

            Self::COMMAND_NOT_ALLOWED => StatusKind::CommandNotAllowed,
            Self::COMMAND_INCOMPATIBLE_FILE_STRUCTURE => {
                StatusKind::CommandIncompatibleFileStructure
            }
            Self::SECURITY_STATUS_NOT_SATISFIED => StatusKind::SecurityStatusNotSatisfied,
            Self::AUTHENTICATION_METHOD_BLOCKED => StatusKind::AuthenticationMethodBlocked,
            Self::REFERENCE_DATA_NOT_USABLE => StatusKind::ReferenceDataNotUsable,
            Self::CONDITION_OF_USE_NOT_SATISFIED => StatusKind::ConditionOfUseNotSatisfied,
            Self::COMMAND_NOT_ALLOWED_NO_EF => StatusKind::CommandNotAllowedNoEf,
            Self::EXECTED_SECURE_MESSAGING_DATA_OBJECTS_MISSING => {
                StatusKind::ExectedSecureMessagingDataObjectsMissing
            }
            Self::INCORRECT_SECURE_MESSAGING_DATA_OBJECTS => {
                StatusKind::IncorrectSecureMessagingDataObjects
            }

            Self::WRONG_PARAMETERS_NO_INFO => StatusKind::WrongParametersNoInfo,
            Self::INCORRECT_PARAMETERS => StatusKind::IncorrectParameters,
            Self::FUNCTION_NOT_SUPPORTED => StatusKind::FunctionNotSupported,
            Self::FILE_OR_APP_NOT_FOUND => StatusKind::FileOrAppNotFound,
            Self::RECORD_NOT_FOUND => StatusKind::RecordNotFound,
            Self::NOT_ENOUGH_MEMORY_IN_FILE => StatusKind::NotEnoughMemoryInFile,
            Self::NC_INCONSISTENT_WITH_TLV => StatusKind::NcInconsistentWithTlv,
            Self::INCORRECT_P1P2 => StatusKind::IncorrectP1p2,
            Self::NC_INCONSISTENT_WITH_P1P2 => StatusKind::NcInconsistentWithP1p2,
            Self::REFERENCE_NOT_FOUND => StatusKind::ReferenceNotFound,
            Self::FILE_ALREADY_EXISTS => StatusKind::FileAlreadyExists,
            Self::DF_NAME_ALREADY_EXISTS => StatusKind::DfNameAlreadyExists,

            Self::WRONG_PARAMETERS => StatusKind::WrongParameters,

            Self::INSTRUCTION_NOT_SUPPORTED_OR_INVALID => {
                StatusKind::InstructionNotSupportedOrInvalid
            }
            Self::CLASS_NOT_SUPPORTED => StatusKind::ClassNotSupported,
            Self::ERROR => StatusKind::Error,
            _ => {
                if let Some(c) = self.as_warning_triggering() {
                    StatusKind::WarningTriggering(c)
                } else if let Some(c) = self.as_error_triggering() {
                    StatusKind::ErrorTriggering(c)
                } else if let Some(a) = self.as_more_available() {
                    StatusKind::MoreAvailable(a)
                } else if let Some(a) = self.as_wrong_le_field() {
                    StatusKind::WrongLeField(a)
                } else if let Some(c) = self.as_warning_counter() {
                    StatusKind::WarningCounter(c)
                } else {
                    return None;
                }
            }
        })
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
/// Expected 0x02 <= value < 0x81
pub struct TriggeringError;

impl Display for TriggeringError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str("Expected 0x02 <= value < 0x81")
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
/// Expected 0x02 <= value < 0x81
pub struct WarningCounterError;

impl Display for WarningCounterError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str("Expected 0x00 <= value < 0xF")
    }
}

impl From<u16> for Status {
    fn from(value: u16) -> Self {
        Self(value)
    }
}

impl From<[u8; 2]> for Status {
    fn from(value: [u8; 2]) -> Self {
        u16::from_be_bytes(value).into()
    }
}

impl From<(u8, u8)> for Status {
    fn from((v1, v2): (u8, u8)) -> Self {
        [v1, v2].into()
    }
}

impl From<Status> for u16 {
    fn from(value: Status) -> Self {
        value.0
    }
}

impl From<Status> for [u8; 2] {
    fn from(value: Status) -> Self {
        value.as_bytes()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn categories() {
        assert!(!Status::SUCCESS.is_more_available());
        assert!(!Status::SUCCESS.is_warning());
        assert!(!Status::SUCCESS.is_warning_without_modification());
        assert!(!Status::SUCCESS.is_warning_with_modification());
        assert!(!Status::SUCCESS.is_execution_error());
        assert!(!Status::SUCCESS.is_checking_error());
        assert!(!Status::SUCCESS.is_error());
        assert!(!Status::SUCCESS.is_warning_triggering());
        assert!(!Status::SUCCESS.is_error_triggering());
        assert!(!Status::SUCCESS.is_wrong_le_field());
        assert!(Status::DATA_UNCHANGED_WARNING.is_warning());
        assert!(Status::DATA_UNCHANGED_WARNING.is_warning_without_modification());
        assert!(!Status::DATA_UNCHANGED_WARNING.is_warning_with_modification());
        assert!(!Status::DATA_UNCHANGED_WARNING.is_error());

        assert!(Status::DATA_CHANGED_WARNING.is_warning());
        assert!(!Status::DATA_CHANGED_WARNING.is_warning_without_modification());
        assert!(Status::DATA_CHANGED_WARNING.is_warning_with_modification());

        assert!(Status::WRONG_LENGTH.is_checking_error());
        assert!(Status::WRONG_LENGTH.is_error());
        assert!(!Status::WRONG_LENGTH.is_execution_error());
        assert!(!Status::WRONG_LENGTH.is_warning());
        assert!(!Status::WRONG_LENGTH.is_wrong_le_field());

        assert!(Status::DATA_CHANGED_ERROR.is_error());
        assert!(Status::DATA_CHANGED_ERROR.is_execution_error());
        assert!(!Status::DATA_CHANGED_ERROR.is_error_triggering());
    }

    #[test]
    fn constructors() {
        for i in 0..u8::MAX {
            let wrong_le = Status::wrong_le_field(i);
            assert!(wrong_le.is_wrong_le_field());
            assert_eq!(wrong_le.as_wrong_le_field().unwrap(), i);

            let more = Status::more_available(i);
            assert!(more.is_more_available());
            assert_eq!(more.as_more_available().unwrap(), i);
        }

        for i in 2..0x81 {
            let trigg = Status::warning_triggering(i);
            assert!(trigg.is_warning_triggering());
            assert_eq!(trigg.as_warning_triggering().unwrap(), i);

            let trigg = Status::error_triggering(i);
            assert!(trigg.is_error_triggering());
            assert_eq!(trigg.as_error_triggering().unwrap(), i);
        }

        for i in 0..0x0F {
            let count = Status::warning_counter(i);
            assert!(count.is_warning_counter());
            assert_eq!(count.as_warning_counter().unwrap(), i);
        }
    }
    #[test]
    fn convert() {
        assert_eq!(Status::SUCCESS.as_u16(), 0x9000);
        assert_eq!(Status::SUCCESS.as_bytes(), [0x90, 0x00]);
    }
}
