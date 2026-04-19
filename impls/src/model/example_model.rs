use base::{define_unit_system, prelude::*};
use crate::{    
    language::{display_text::CommonDisplayText, us::UnitedStatesLanguage}, 
    model::{component::id::ID64, 
    registry_hashmap::HashMapRegistry
}}; 

// 1. Define the Unit System
// This creates two enums under the hood.  
define_unit_system!(ExampleUnitCategory, ExampleUnitSetting {
    length:       Length:      Simple,
    length_small: LengthSmall: Simple,
    area:         Area:        Simple,
    force:        Force:       Compound,
    custom:       Custom:      Compound,
});
// the macro above will make 
// // 1. The ID Enum (Used to identify WHICH category we are talking about)
// #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
// pub enum ExampleUnitCategory {
//     Length,
//     LengthSmall,
//     Area,
//     Force,
//     Custom,
// }

// // Implements your marker trait
// impl crate::unit::UnitCategory for ExampleUnitCategory {}

// // 2. The Storage Struct (Used to store the actual SELECTION for each category)
// #[derive(Debug, Clone, Copy, PartialEq)]
// pub struct ExampleUnitSetting {
//     pub length: crate::unit::SimpleUnit,
//     pub length_small: crate::unit::SimpleUnit,
//     pub area: crate::unit::SimpleUnit,
//     pub force: crate::unit::CompoundUnit,
//     pub custom: crate::unit::CompoundUnit,
// }

// // 3. The Mapping Logic (The bridge between ID and Data)
// impl crate::unit::UnitCategoryKind<ExampleUnitCategory> for ExampleUnitSetting {
//     fn get_kind(&self, category: ExampleUnitCategory) -> crate::unit::UnitKind {
//         match category {
//             ExampleUnitCategory::Length => {
//                 crate::unit::UnitKind::SimpleUnit(self.length)
//             }
//             ExampleUnitCategory::LengthSmall => {
//                 crate::unit::UnitKind::SimpleUnit(self.length_small)
//             }
//             ExampleUnitCategory::Area => {
//                 crate::unit::UnitKind::SimpleUnit(self.area)
//             }
//             ExampleUnitCategory::Force => {
//                 crate::unit::UnitKind::CompoundUnit(self.force)
//             }
//             ExampleUnitCategory::Custom => {
//                 crate::unit::UnitKind::CompoundUnit(self.custom)
//             }
//         }
//     }
// }

// 2. Add a standard initialization of the unit setting for this project
impl ExampleUnitSetting {
    pub const fn new() -> Self {
        Self {
            length: SimpleUnit::length(),
            
            length_small: SimpleUnit::Length { 
                unit: LengthUnit::Millimeter, 
                exponent: 1 
            },
            
            area: SimpleUnit::area(),
            force: CompoundUnit::force(),
            custom: CompoundUnit::new(),
        }
    }
}

// COMPONENT ID
// This is the ID to use for components. You can use unsigned integers, like usize or u64,
// but for more type saftey use an ID type.  Some ID types are defined but you can define your own
// with: component_id_macro!(ID64, u64);

/// COMPONENT KINDS
/// The components in the model are of different kinds, or types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ExampleKind {
    Point,
    Line,
}

impl ComponentKind for ExampleKind {
    type Id = ID64; // This tells the Registry/Model to expect ID64
}

// COMPONENT DATA
// Each component kind has associated data. There should be a one to one mapping
// of the kind enum and data enum
#[derive(Debug, Clone, Copy, PartialEq)] 
pub enum ExampleData {
    Point {
        x: f64,
        y: f64,
        z: f64,
    },
    Line {
        i_id: ID64, 
        j_id: ID64, 
    },
}

impl ComponentData for ExampleData {
    type Kind = ExampleKind;

    fn kind(&self) -> Self::Kind {
        match self {
            Self::Point { .. } => ExampleKind::Point,
            Self::Line { .. } => ExampleKind::Line,
        }
    }
}

// COMPONENT REGISTRY
// Components are stored in a registry

// CONFIG
// This allows us to define all the generic types the model will use in one place
pub struct ExampleConfig;

impl ModelConfig for ExampleConfig {
    type Id = ID64;
    type Data = ExampleData;
    type Registry = HashMapRegistry<Self::Data>;

    // Use '=' to assign the concrete types
    type UnitCategory = ExampleUnitCategory;
    type UnitSetting = ExampleUnitSetting; // This struct must implement UnitSetting trait
 
    type Display = CommonDisplayText;
    type Lang = UnitedStatesLanguage;
}

// MODEL
// Now you can make the model using the configuration
pub type ExampleModel = Model<ExampleConfig>;

// INSTANTIATION

// 4. Instantiation Example
fn example_main() {
    let registry = HashMapRegistry::<ExampleData>::default();
    
    // Create settings: File is standard SI, Display uses specific preferences
    let file_settings = ExampleUnitSetting::new();
    let mut display_settings = ExampleUnitSetting::new();
    
    // Customize the display setting: show large lengths in feet
    display_settings.length = SimpleUnit::Length { 
        unit: LengthUnit::Foot, 
        exponent: 1 
    };

    let settings = UnitSystem::<ExampleConfig>::new(file_settings, display_settings);
    let mut model = Model::<ExampleConfig>::new(registry, settings);

    // Now, any property using ExampleUnitCategory::Length will 
    // automatically convert Meters (file) to Feet (display).
}
