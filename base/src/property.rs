pub mod error;
pub mod value;
pub mod propertied;
pub mod node;


pub use value::*;
pub use error::*;
pub use propertied::*;
pub use node::*;

use crate::{model::ModelConfig, prelude::{DisplayText, Language}, unit::{UnitSettings, UnitSystem}}; 

pub trait PropertyConfig: ModelConfig {
    type Display: DisplayText;  
    type Lang: Language; 
}

pub struct Property<C: PropertyConfig> {
    pub name: C::Display,
    pub value: PropertyValue,  
    pub unit: Option<C::UnitCategory>,    
}

impl<C: PropertyConfig> Property<C> {
    pub fn parse_value(&self, input: &str, system: &UnitSystem<C>) -> PropertyValue {
        match &self.value {
            PropertyValue::Number(_) => {
                let val: f64 = input.parse().unwrap_or(0.0);
                if let Some(cat) = self.unit {
                    // Convert from Display unit to SI Base
                    PropertyValue::Number(system.display.get(cat).to_base(val))
                } else {
                    PropertyValue::Number(val)
                }
            }
            PropertyValue::Percent(_) => {
                // Percents are typically 0.0-1.0 internally; 
                // user types "50", we store 0.5
                let val: f64 = input.parse().unwrap_or(0.0);
                PropertyValue::Percent(val / 100.0)
            }
            PropertyValue::Integer(_) => PropertyValue::Integer(input.parse().unwrap_or(0)),
            PropertyValue::Boolean(_) => {
                let s = input.to_lowercase();
                PropertyValue::Boolean(s == "true" || s == "1" || s == "yes")
            }
            PropertyValue::Text(_) => PropertyValue::Text(input.to_string()),
        }
    }

    pub fn format_value(&self, value: PropertyValue, system: &UnitSystem<C>) -> String {
        match (&self.value, value) {
            (PropertyValue::Number(_), PropertyValue::Number(n)) => {
                if let Some(cat) = self.unit {
                    let display_kind = system.display.get(cat);
                    let converted = display_kind.from_base(n);
                    format!("{:.2} {}", converted, system.symbol(cat))
                } else {
                    format!("{:.2}", n)
                }
            }
            (PropertyValue::Percent(_), PropertyValue::Percent(n)) => {
                format!("{:.1}%", n * 100.0)
            }
            (PropertyValue::Integer(_), PropertyValue::Integer(i)) => i.to_string(),
            (PropertyValue::Boolean(_), PropertyValue::Boolean(b)) => b.to_string(),
            (PropertyValue::Text(_), PropertyValue::Text(t)) => t,
            // Fallback for mismatched types or "None" states
            _ => "---".to_string(),
        }
    }
}

 

// pub struct Property<K: UnitCategory> {
//     pub name: DisplayText,
//     pub key: Option<K>,
//     // Now extracts from any object that implements Propertied for this config
//     pub extractor: fn(&dyn Propertied<K>) -> PropertyValue,
// }


 