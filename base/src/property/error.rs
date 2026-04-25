

use crate::property::value::PropertyValueKind;

#[derive(Debug, PartialEq)]
pub enum PropertyError {
    NotFound(String),
    TypeMismatch { expected: PropertyValueKind, got: PropertyValueKind },
    ParseFailed { expected: PropertyValueKind, raw: String },
    InvalidValue(String),
    ReadOnly(String),
}

impl std::fmt::Display for PropertyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotFound(name) => 
                write!(f, "Property '{}' not found", name),
            Self::TypeMismatch { expected, got } => 
                write!(f, "Type mismatch: expected {}, got {}", expected, got),
            Self::ParseFailed { expected, raw } => 
                write!(f, "Could not parse '{}' as {}", raw, expected),
            Self::InvalidValue(msg) => 
                write!(f, "Invalid value: {}", msg),
            Self::ReadOnly(name) => 
                write!(f, "Property '{}' is read-only", name),
        }
    }
}