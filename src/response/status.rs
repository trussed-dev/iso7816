use core::fmt::{Debug, Display};

#[derive(Copy, Clone, Eq, PartialEq, PartialOrd, Ord)]
pub struct Status(pub u16);

impl Default for Status {
    fn default() -> Self {
        Self::SUCCESS
    }
}

impl Debug for Status {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match *self {
            Self::SUCCESS => f.write_str("SUCCESS"),

            Self::DATA_UNCHANGED_WARNING => f.write_str("DATA_UNCHANGED_WARNING"),
            Self::CORRUPTED_DATA => f.write_str("CORRUPTED_DATA"),
            Self::UNEXPECTED_EOF => f.write_str("UNEXPECTED_EOF"),
            Self::SELECT_FILE_DEACTIVATED => f.write_str("SELECT_FILE_DEACTIVATED"),
            Self::FILE_CONTROL_INFO_BADLY_FORMATTED => {
                f.write_str("FILE_CONTROL_INFO_BADLY_FORMATTED")
            }
            Self::SELECT_FILE_IN_TERMINATION_STATE => {
                f.write_str("SELECT_FILE_IN_TERMINATION_STATE")
            }
            Self::NO_INPUT_DATA_FROM_SENSOR => f.write_str("NO_INPUT_DATA_FROM_SENSOR"),

            Self::DATA_CHANGED_WARNING => f.write_str("DATA_CHANGED_WARNING"),
            Self::FILLED_BY_LAST_WRITE => f.write_str("FILLED_BY_LAST_WRITE"),

            Self::DATA_CHANGED_ERROR => f.write_str("DATA_CHANGED_ERROR"),
            Self::MEMORY_FAILURE => f.write_str("MEMORY_FAILURE"),

            Self::CLA_NOT_SUPPORTED => f.write_str("CLA_NOT_SUPPORTED"),
            Self::LOGICAL_CHANNEL_NOT_SUPPORTED => f.write_str("LOGICAL_CHANNEL_NOT_SUPPORTED"),
            Self::SECURE_MESSAGING_NOT_SUPPORTED => f.write_str("SECURE_MESSAGING_NOT_SUPPORTED"),
            Self::LAST_COMMANND_OF_CHAIN_EXPECTED => f.write_str("LAST_COMMANND_OF_CHAIN_EXPECTED"),
            Self::COMMAND_CHAINING_NOT_SUPPORTED => f.write_str("COMMAND_CHAINING_NOT_SUPPORTED"),

            Self::COMMAND_NOT_ALLOWED => f.write_str("COMMAND_NOT_ALLOWED"),
            Self::COMMAND_INCOMPATIBLE_FILE_STRUCTURE => {
                f.write_str("COMMAND_INCOMPATIBLE_FILE_STRUCTURE")
            }
            Self::SECURITY_STATUS_NOT_SATISFIED => f.write_str("SECURITY_STATUS_NOT_SATISFIED"),
            Self::AUTHENTICATION_METHOD_BLOCKED => f.write_str("AUTHENTICATION_METHOD_BLOCKED"),
            Self::REFERENCE_DATA_NOT_USABLE => f.write_str("REFERENCE_DATA_NOT_USABLE"),
            Self::CONDITION_OF_USE_NOT_SATISFIED => f.write_str("CONDITION_OF_USE_NOT_SATISFIED"),
            Self::COMMAND_NOT_ALLOWED_NO_EF => f.write_str("COMMAND_NOT_ALLOWED_NO_EF"),
            Self::EXECTED_SECURE_MESSAGING_DATA_OBJECTS_MISSING => {
                f.write_str("EXECTED_SECURE_MESSAGING_DATA_OBJECTS_MISSING")
            }
            Self::INCORRECT_SECURE_MESSAGING_DATA_OBJECTS => {
                f.write_str("INCORRECT_SECURE_MESSAGING_DATA_OBJECTS")
            }

            Self::WRONG_PARAMETERS_NO_INFO => f.write_str("WRONG_PARAMETERS_NO_INFO"),
            Self::INCORRECT_PARAMETERS => f.write_str("INCORRECT_PARAMETERS"),
            Self::FUCNTION_NOT_SUPPORTED => f.write_str("FUCNTION_NOT_SUPPORTED"),
            Self::FILE_OR_APP_NOT_FOUND => f.write_str("FILE_OR_APP_NOT_FOUND"),
            Self::RECORD_NOT_FOUND => f.write_str("RECORD_NOT_FOUND"),
            Self::NOT_ENOUGH_MEMORY_IN_FILE => f.write_str("NOT_ENOUGH_MEMORY_IN_FILE"),
            Self::NC_INCONSISTENT_WITH_TLV => f.write_str("NC_INCONSISTENT_WITH_TLV"),
            Self::INCORRECT_P1P2 => f.write_str("INCORRECT_P1P2"),
            Self::NC_INCONSISTENT_WITH_P1P2 => f.write_str("NC_INCONSISTENT_WITH_P1P2"),
            Self::REFERENCE_NOT_FOUND => f.write_str("REFERENCE_NOT_FOUND"),
            Self::FILE_ALREADY_EXISTS => f.write_str("FILE_ALREADY_EXISTS"),
            Self::DF_NAME_ALREADY_EXISTS => f.write_str("DF_NAME_ALREADY_EXISTS"),

            Self::WRONG_PARAMETERS => f.write_str("WRONG_PARAMETERS"),

            Self::INSTRUCTION_NOT_SUPPORTED_OR_INVALID => {
                f.write_str("INSTRUCTION_NOT_SUPPORTED_OR_INVALID")
            }
            Self::CLASS_NOT_SUPPORTED => f.write_str("CLASS_NOT_SUPPORTED"),
            Self::ERROR => f.write_str("ERROR"),
            _ => {
                if let Some(c) = self.as_warning_triggering() {
                    f.debug_struct("WARNING_TRIGGERING")
                        .field("query_length", &c)
                        .finish()
                } else if let Some(c) = self.as_error_triggering() {
                    f.debug_struct("ERROR_TRIGGERING")
                        .field("query_length", &c)
                        .finish()
                } else if let Some(a) = self.as_more_available() {
                    f.debug_struct("MORE_AVAILABLE")
                        .field("available", &a)
                        .finish()
                } else if let Some(a) = self.as_wrong_le_field() {
                    f.debug_struct("WRONG_LE_FIELD")
                        .field("available", &a)
                        .finish()
                } else if let Some(c) = self.as_warning_counter() {
                    f.debug_struct("WARNING_COUNTER")
                        .field("counter", &c)
                        .finish()
                } else {
                    f.write_fmt(format_args!("{:02x}", self.0))
                }
            }
        }
    }
}

impl Status {
    pub const SUCCESS: Self = Self(0x9000);

    const MORE_AVAILABLE_MASK: u16 = 0x6100;

    pub const DATA_UNCHANGED_WARNING: Self = Self(0x6200);
    const WARNING_TRIGGERING_LOWER: u16 = 0x6202;
    const WARNING_TRIGGERING_UPPER: u16 = 0x6280;
    const ERROR_TRIGGERING_LOWER: u16 = 0x6402;
    const ERROR_TRIGGERING_UPPER: u16 = 0x6480;
    pub const CORRUPTED_DATA: Self = Self(0x6281);
    pub const UNEXPECTED_EOF: Self = Self(0x6282);
    pub const SELECT_FILE_DEACTIVATED: Self = Self(0x6283);
    pub const FILE_CONTROL_INFO_BADLY_FORMATTED: Self = Self(0x6284);
    pub const SELECT_FILE_IN_TERMINATION_STATE: Self = Self(0x6285);
    pub const NO_INPUT_DATA_FROM_SENSOR: Self = Self(0x6286);

    pub const DATA_CHANGED_WARNING: Self = Self(0x6300);
    pub const FILLED_BY_LAST_WRITE: Self = Self(0x6381);
    const WARNING_COUNTER_MASK: u16 = 0x63C0;

    pub const DATA_CHANGED_ERROR: Self = Self(0x6500);
    pub const MEMORY_FAILURE: Self = Self(0x6581);

    pub const CLA_NOT_SUPPORTED: Self = Self(0x6800);
    pub const LOGICAL_CHANNEL_NOT_SUPPORTED: Self = Self(0x6881);
    pub const SECURE_MESSAGING_NOT_SUPPORTED: Self = Self(0x6882);
    pub const LAST_COMMANND_OF_CHAIN_EXPECTED: Self = Self(0x6883);
    pub const COMMAND_CHAINING_NOT_SUPPORTED: Self = Self(0x6884);

    pub const COMMAND_NOT_ALLOWED: Self = Self(0x6900);
    pub const COMMAND_INCOMPATIBLE_FILE_STRUCTURE: Self = Self(0x6981);
    pub const SECURITY_STATUS_NOT_SATISFIED: Self = Self(0x6982);
    pub const AUTHENTICATION_METHOD_BLOCKED: Self = Self(0x6983);
    pub const REFERENCE_DATA_NOT_USABLE: Self = Self(0x6984);
    pub const CONDITION_OF_USE_NOT_SATISFIED: Self = Self(0x6985);
    pub const COMMAND_NOT_ALLOWED_NO_EF: Self = Self(0x6986);
    pub const EXECTED_SECURE_MESSAGING_DATA_OBJECTS_MISSING: Self = Self(0x6987);
    pub const INCORRECT_SECURE_MESSAGING_DATA_OBJECTS: Self = Self(0x6988);

    pub const WRONG_PARAMETERS_NO_INFO: Self = Self(0x6A00);
    pub const INCORRECT_PARAMETERS: Self = Self(0x6A80);
    pub const FUCNTION_NOT_SUPPORTED: Self = Self(0x6A81);
    pub const FILE_OR_APP_NOT_FOUND: Self = Self(0x6A82);
    pub const RECORD_NOT_FOUND: Self = Self(0x6A83);
    pub const NOT_ENOUGH_MEMORY_IN_FILE: Self = Self(0x6A84);
    pub const NC_INCONSISTENT_WITH_TLV: Self = Self(0x6A85);
    pub const INCORRECT_P1P2: Self = Self(0x6A86);
    pub const NC_INCONSISTENT_WITH_P1P2: Self = Self(0x6A87);
    pub const REFERENCE_NOT_FOUND: Self = Self(0x6A88);
    pub const FILE_ALREADY_EXISTS: Self = Self(0x6A89);
    pub const DF_NAME_ALREADY_EXISTS: Self = Self(0x6A8A);

    pub const WRONG_PARAMETERS: Self = Self(0x6B00);

    const WRONG_LE_FIELD_MASK: u16 = 0x6C00;

    pub const INSTRUCTION_NOT_SUPPORTED_OR_INVALID: Self = Self(0x6D00);
    pub const CLASS_NOT_SUPPORTED: Self = Self(0x6E00);
    pub const ERROR: Self = Self(0x6F00);

    pub const fn as_more_available(self) -> Option<u8> {
        if self.0 | Self::MORE_AVAILABLE_MASK == Self::MORE_AVAILABLE_MASK {
            Some((self.0 & 0x00FF) as u8)
        } else {
            None
        }
    }
    pub const fn is_more_available(self) -> bool {
        self.as_more_available().is_some()
    }

    pub const fn more_available(value: u16) -> Self {
        Self(Self::MORE_AVAILABLE_MASK | value as u16)
    }

    pub const fn is_warning(self) -> bool {
        self.is_warning_without_modification() || self.is_warning_with_modification()
    }

    /// The proccessing raised a warning and did not change state
    pub const fn is_warning_without_modification(self) -> bool {
        self.0 | 0x6200 == 0x6200
    }
    /// The proccessing raised a warning and changed state
    pub const fn is_warning_with_modification(self) -> bool {
        self.0 | 0x6300 == 0x6300
    }

    pub const fn is_execution_error(self) -> bool {
        self.0 | 0x6400 == 0x6400 || self.0 | 0x6500 == 0x6500 || self.0 | 0x6600 == 0x6600
    }
    pub const fn is_checking_error(self) -> bool {
        self.0 | 0x6700 == 0x6700
            || self.0 | 0x6800 == 0x6800
            || self.0 | 0x6900 == 0x6900
            || self.0 | 0x6A00 == 0x6A00
            || self.0 | 0x6B00 == 0x6B00
            || self.0 | 0x6C00 == 0x6C00
            || self.0 | 0x6D00 == 0x6D00
            || self.0 | 0x6E00 == 0x6E00
            || self.0 | 0x6F00 == 0x6F00
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

    /// Value must be 0x02 <= value < 0x81, otherwise panics
    pub const fn warning_triggering(value: u8) -> Self {
        match Self::try_warning_triggering(value) {
            Ok(s) => s,
            Err(_) => panic!("Expected 0x02 <= value < 0x81"),
        }
    }
    /// Value must be 0x02 <= value < 0x81, otherwise panics
    pub const fn try_warning_triggering(value: u8) -> Result<Self, TriggeringError> {
        if value <= 0x80 && value >= 0x02 {
            Ok(Self(Self::WARNING_TRIGGERING_LOWER | value as u16))
        } else {
            Err(TriggeringError)
        }
    }

    pub const fn is_warning_counter(self) -> bool {
        self.as_warning_counter().is_some()
    }

    pub const fn as_warning_counter(self) -> Option<u8> {
        if self.0 | Self::WARNING_COUNTER_MASK == Self::WARNING_COUNTER_MASK {
            Some((self.0 | 0x00F) as u8)
        } else {
            None
        }
    }

    pub const fn try_warning_counter(value: u8) -> Result<Self, WarningCounterError> {
        if value <= 0xF {
            Ok(Self(Self::WARNING_COUNTER_MASK | value as u16))
        } else {
            Err(WarningCounterError)
        }
    }
    /// Value must be 0x00 <= value < 0x0F, otherwise panics
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

    /// Value must be 0x02 <= value < 0x81, otherwise panics
    pub const fn error_triggering(value: u8) -> Self {
        match Self::try_error_triggering(value) {
            Ok(s) => s,
            Err(_) => panic!("Expected 0x02 <= value < 0x81"),
        }
    }
    /// Value must be 0x02 <= value < 0x81, otherwise panics
    pub const fn try_error_triggering(value: u8) -> Result<Self, TriggeringError> {
        if value <= 0x80 && value >= 0x02 {
            Ok(Self(Self::ERROR_TRIGGERING_LOWER | value as u16))
        } else {
            Err(TriggeringError)
        }
    }

    pub const fn as_wrong_le_field(self) -> Option<u8> {
        if self.0 | Self::WRONG_LE_FIELD_MASK == Self::WRONG_LE_FIELD_MASK {
            Some((self.0 & 0x00FF) as u8)
        } else {
            None
        }
    }
    pub const fn is_wrong_le_field(self) -> bool {
        self.as_wrong_le_field().is_some()
    }
    pub const fn wrong_le_field(available_bytes: u8) -> Self {
        Self(Self::WRONG_LE_FIELD_MASK | available_bytes as u16)
    }

    pub const fn as_bytes(self) -> [u8; 2] {
        self.0.to_be_bytes()
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
