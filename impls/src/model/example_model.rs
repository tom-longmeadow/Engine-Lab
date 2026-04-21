use base::prelude::*;

use crate::model::registry_hashmap::HashMapRegistry;
 


/// UNITS
/// Define the unit categories.  
/// The model may need small and large units of the same base type.  For instance
/// you may have large components measured in meters but small components measured
/// in millimeters.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ExampleUnitCategory {
    Length,
    LengthSmall,
    Area,
    Force,
    DynamicViscosity,
    Temperature,
}

/// Implement the UnitCategory trait to mark this as a unit category
impl UnitCategory for ExampleUnitCategory {}

/// Define the kind (type) of unit for each categories. 
/// SimpleUnit contains a base unit and exponent for a unit, like mm^2
/// CompoundUnit contains SimpleUnits for each base unit, like kg*m/s^2
/// TemperatureUnit is special and different than other units.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ExampleUnitSettings {
    pub length: SimpleUnit,
    pub length_small: SimpleUnit,
    pub area: SimpleUnit,
    pub force: CompoundUnit,
    pub dynamic_viscosity: CompoundUnit,
    pub temperature: TemperatureUnit
}

/// Map the unit category to a particular unit
/// You can use helper functions for defaults, 
/// or make the unit yourself.
impl UnitSettings<ExampleUnitCategory> for ExampleUnitSettings {
   
    fn default() -> Self {
        Self {
            length: SimpleUnit::length_si(), // use the si default
            length_small: SimpleUnit::length(LengthUnit::Millimeter, 1), // set to millimeter
            area: SimpleUnit::area_si(),
            force: CompoundUnit::force(),
            
            dynamic_viscosity: CompoundUnit::new()// Dynamic Viscosity: kg / (m · s)
                .with_mass(MassUnit::Kilogram, 1)
                .with_length(LengthUnit::Meter, -1)
                .with_time(TimeUnit::Second, -1),
            temperature: TemperatureUnit::Celsius,
        }
    }

    // 2. Map the category to the specific field wrapped in UnitKind
    fn get(&self, category: ExampleUnitCategory) -> UnitKind {
        match category {
            ExampleUnitCategory::Length      => UnitKind::Simple(self.length),
            ExampleUnitCategory::LengthSmall => UnitKind::Simple(self.length_small),
            ExampleUnitCategory::Area        => UnitKind::Simple(self.area),
            ExampleUnitCategory::Force       => UnitKind::Compound(self.force),
            ExampleUnitCategory::DynamicViscosity      => UnitKind::Compound(self.dynamic_viscosity),
            ExampleUnitCategory::Temperature => UnitKind::Temperature(self.temperature),
        }
    }
     
}
 
// COMPONENT ID
// This is the ID to use for components. You can use unsigned integers, like usize or u64,
// but for more type saftey use an ID type.  Some ID types are defined but you can define your own.
// We have defined ID64 as an u64, and have made a using statement above for it, but if you wanted a 
// custom id you could do it with this macro
component_id_macro!(MyName, u32);  

/// COMPONENT KINDS
/// The components in the model are of different kinds, or types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ExampleKind {
    Point,
    Line,
}

impl ComponentKind for ExampleKind {
    type Id = ID64; // This tells the Registry/Model to expect ID64 as the ComponentId
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
        i_id: ID64, // make sure to use the Id defined in ExampleKind
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
// for this example we will use a HasMapResgistry alread defined.

// CONFIG
// This allows us to define all the generic types the model will use in one place
pub struct ExampleConfig;

impl ModelConfig for ExampleConfig {
    type Id = ID64;
    type Data = ExampleData;
    type Registry = HashMapRegistry<Self::Data>;

    // Use '=' to assign the concrete types
    type UnitCategory = ExampleUnitCategory;
    type UnitSetting = ExampleUnitSettings;  
 
    // type Display = CommonDisplayText;
    // type Lang = UnitedStatesLanguage;
}

// MODEL
// Now you can make the model using the configuration
pub type ExampleModel = Model<ExampleConfig>;

// INSTANTIATION

// 4. Instantiation Example
fn example_main() {
    let registry = HashMapRegistry::<ExampleData>::default();
    
    // Create settings: File is standard SI, Display uses specific preferences
    let file_settings = ExampleUnitSettings::default();
    let mut display_settings = ExampleUnitSettings::default();
    
    // Customize the display setting: show large lengths in feet
    display_settings.length = SimpleUnit::Length { 
        unit: LengthUnit::Foot, 
        exponent: 1 
    };

    let settings = UnitSystem::<ExampleConfig>::new(file_settings, display_settings);
    let mut model = Model::<ExampleConfig>::new(registry, settings);

     
}
