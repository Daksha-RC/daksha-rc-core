use std::fmt;
use cqrs_es::DomainEvent;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SchemaDefEvent {
    DefCreated {
        id: String,
        schema: String,
    },
    DefCreatedAndValidated {
        id: String,
        schema: String,
    },
    DefValidated{
        id: String,
    },
    DefActivated{
        id: String,
    },
    DefDeactivated{
        id: String,
    },
}

impl DomainEvent for SchemaDefEvent {
    fn event_type(&self) -> String {
        let event_type: &str = match self {
            SchemaDefEvent::DefCreated { .. } => "DefCreated",
            SchemaDefEvent::DefValidated { .. } => "DefValidated",
            SchemaDefEvent::DefActivated { .. } => "DefActivated",
            SchemaDefEvent::DefDeactivated { .. } => "DefDeactivated",
            SchemaDefEvent::DefCreatedAndValidated { .. } => "DefCreatedAndValidated",
        };
        event_type.to_string()
    }

    fn event_version(&self) -> String {
        "1.0".to_string()
    }
}

#[derive(Debug,Serialize)]
pub enum SchemaDefError {
    ExistsError { error_message: String, error_code: u32 },
    ValidationError { error_message: String, error_code: u32 },
    ActivationError { error_message: String, error_code: u32 },
    DeactivationError { error_message: String, error_code: u32 },
    GeneralError { error_message: String, error_code: u32 },
}
impl fmt::Display for SchemaDefError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SchemaDefError::ExistsError { error_message: message, error_code: code } => write!(f, "ExistsError {}: {}", code, message),
            SchemaDefError::ValidationError { error_message: message, error_code: code } => write!(f, "ValidationError {}: {}", code, message),
            SchemaDefError::ActivationError { error_message: message, error_code: code } => write!(f, "ActivationError {}: {}", code, message),
            SchemaDefError::DeactivationError { error_message: message, error_code: code } => write!(f, "DeactivationError {}: {}", code, message),
            SchemaDefError::GeneralError { error_message: message, error_code: code } => write!(f, "GeneralError {}: {}", code, message),
        }
    }
}
impl std::error::Error for SchemaDefError {}
//
// #[derive(Debug)]
// pub struct SchemaDefExistsError {
//     pub message: String,
//     pub code: u32,
// }
//
// impl fmt::Display for SchemaDefExistsError {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "Error {}: {}", self.code, self.message)
//     }
// }
//
// impl std::error::Error for SchemaDefExistsError {}
//
// #[derive(Debug)]
// pub struct SchemaDefError {
//     pub message: String,
//     pub code: u32,
// }
//
// impl fmt::Display for SchemaDefError {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "Error {}: {}", self.code, self.message)
//     }
// }
//
// impl std::error::Error for SchemaDefError {}