use serde::ser::SerializeStruct;
use serde::Serialize;
use std::convert::Infallible;
use std::fmt;
extern crate alloc;

//TODO: Utilize other error types more
#[derive(Debug)]
pub enum AvailErrorType {
    Internal,
    External,
    Database,
    LocalStorage,
    NotFound,
    InvalidData,
    Validation,
    Network,
    File,
}

impl fmt::Display for AvailErrorType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let str_value = match &self {
            AvailErrorType::Internal => "Internal",
            AvailErrorType::External => "External",
            AvailErrorType::Database => "Database",
            AvailErrorType::NotFound => "Not Found",
            AvailErrorType::InvalidData => "Invalid Data",
            AvailErrorType::Validation => "Validation",
            AvailErrorType::LocalStorage => "Local Storage",
            AvailErrorType::Network => "Network",
            AvailErrorType::File => "File",
        };

        write!(f, "{}", str_value)
    }
}

#[derive(Debug)]
pub struct AvailError {
    pub error_type: AvailErrorType,
    pub internal_msg: String,
    pub external_msg: String,
}

impl AvailError {
    pub fn new(
        error_type: AvailErrorType,
        internal_msg: String,
        external_msg: String,
    ) -> AvailError {
        AvailError {
            error_type,
            internal_msg,
            external_msg,
        }
    }
}

impl From<serde_json::Error> for AvailError {
    fn from(value: serde_json::Error) -> Self {
        Self {
            error_type: AvailErrorType::InvalidData,
            internal_msg: format!("SerdeJsonError: {}", value),
            external_msg: "Invalid JSON".to_string(),
        }
    }
}

impl From<std::num::TryFromIntError> for AvailError {
    fn from(value: std::num::TryFromIntError) -> Self {
        Self {
            error_type: AvailErrorType::InvalidData,
            internal_msg: format!("TryFromIntError: {}", value),
            external_msg: "Invalid TryFromIntError".to_string(),
        }
    }
}

impl From<snarkvm::prelude::bech32::Error> for AvailError {
    fn from(value: snarkvm::prelude::bech32::Error) -> Self {
        Self {
            error_type: AvailErrorType::InvalidData,
            internal_msg: format!("Bech32Error: {}", value),
            external_msg: "Invalid Bech32".to_string(),
        }
    }
}

impl From<alloc::string::FromUtf8Error> for AvailError {
    fn from(value: alloc::string::FromUtf8Error) -> Self {
        Self {
            error_type: AvailErrorType::InvalidData,
            internal_msg: format!("FromUtf8Error: {}", value),
            external_msg: "Invalid UTF8".to_string(),
        }
    }
}

impl From<std::io::Error> for AvailError {
    fn from(value: std::io::Error) -> Self {
        Self {
            error_type: AvailErrorType::File,
            internal_msg: format!("IOError: {}", value),
            external_msg: "Microservice initialization Fail".to_string(),
        }
    }
}

impl From<std::convert::Infallible> for AvailError {
    fn from(value: Infallible) -> Self {
        Self {
            error_type: AvailErrorType::Internal,
            internal_msg: format!("Infallible: {}", value),
            external_msg: "Internal error".to_string(),
        }
    }
}

impl From<std::num::ParseIntError> for AvailError {
    fn from(value: std::num::ParseIntError) -> Self {
        Self {
            error_type: AvailErrorType::InvalidData,
            internal_msg: format!("ParseIntError: {}", value),
            external_msg: "Invalid Int".to_string(),
        }
    }
}

pub type AvailResult<T> = Result<T, AvailError>;

impl fmt::Display for AvailError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Type: '{}' | Internal Msg: '{}' | External Msg: '{}'",
            self.error_type, self.internal_msg, self.external_msg
        )
    }
}

impl Serialize for AvailErrorType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

pub type AvError = AvailError;

impl Serialize for AvailError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("AvailError", 3)?;
        state.serialize_field("error_type", &self.error_type)?;
        state.serialize_field("internal_msg", &self.internal_msg)?;
        state.serialize_field("external_msg", &self.external_msg)?;
        state.end()
    }
}
