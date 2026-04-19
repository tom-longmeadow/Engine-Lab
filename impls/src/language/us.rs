

use base::prelude::Language;

/// An example of how to define your supported languages
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum UnitedStatesLanguage {
    #[default]
    English,
}

impl Language for UnitedStatesLanguage {
    fn id(&self) -> &'static str {
        match self {
            Self::English => "en-US", 
        }
    }
}