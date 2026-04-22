use crate::prelude::{
        DisplayLanguage, DisplayText, Language, UnitConfig
    }; 

pub trait PropertyConfig: UnitConfig {
    type Display: DisplayLanguage + Clone + From<DisplayText> + Into<DisplayText>;  
    type Lang: Language; 
}
