
use crate::prelude::DisplayLanguage;

use super::{
    PropertyConfig
};


#[derive(Debug, Clone, PartialEq, Eq)] 
pub enum PropertyName<C: PropertyConfig> {
    Localized(C::Display),
    Raw(String),
}

impl<C: PropertyConfig> PropertyName<C> {
    /// Ergonomic helper for localized keys
    pub fn localized(key: C::Display) -> Self {
        Self::Localized(key)
    }

    /// Ergonomic helper for raw strings
    pub fn raw(s: impl Into<String>) -> Self {
        Self::Raw(s.into())
    }

    pub fn to_string(&self) -> String {
        match self {
            Self::Localized(key) => key.default_text().to_string(),
            Self::Raw(s) => s.clone(),  
        }
    }
}