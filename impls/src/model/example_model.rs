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

// impl<C: ModelConfig> Propertied<C> for ExampleData {
//     fn get_template() -> Vec<PropertyNode<C>> {
//         // Return only the properties specific to Point or Line
//         vec![/* property nodes for x, y, z or i_id, j_id */]
//     }

//     fn get_value(&self, prop: &Property<C>) -> PropertyValue {
//         // Match on prop and return self.x, self.y, etc.
//     }

//     fn set_value(&mut self, prop: &Property<C>, value: PropertyValue) -> Result<(), PropertyError> {
//         // Update self.x, self.y, etc.
//         Ok(())
//     }
// }

// impl ComponentData for ExampleData {
//     type Kind = ExampleKind;

//     fn kind(&self) -> Self::Kind {
//         match self {
//             Self::Point { .. } => ExampleKind::Point,
//             Self::Line { .. } => ExampleKind::Line,
//         }
//     }
// }

// impl<C, D> Propertied<C> for Component<D>
// where
//     C: ModelConfig,
//     D: ComponentData + Propertied<C>,
// {
//     fn get_template() -> Vec<PropertyNode<C>> {
//         // 1. Create the template for the ID property
//         let mut template = vec![ PropertyNode::new_id_property() ];
        
//         // 2. Append all the properties from the inner data
//         template.extend(D::get_template());
        
//         template
//     }

//     fn get_value(&self, prop: &Property<C>) -> PropertyValue {
//         if prop.is_id_property() {
//             // Intercept and return the component's actual ID
//             return PropertyValue::Id(self.id);
//         }
//         // Otherwise, delegate to the inner data
//         self.data.get_value(prop)
//     }

//     fn set_value(&mut self, prop: &Property<C>, value: PropertyValue) -> Result<(), PropertyError> {
//         if prop.is_id_property() {
//             // Prevent changing the ID if it's read-only, or handle it here
//             return Err(PropertyError::ReadOnly);
//         }
//         // Otherwise, delegate the mutation to the inner data
//         self.data.set_value(prop, value)
//     }
// }

// COMPONENT REGISTRY
// Components are stored in a registry
// for this example we will use a HasMapResgistry alread defined.

// CONFIG
// This allows us to define all the generic types the model will use in one place
// pub struct ExampleConfig;

// impl ModelConfig for ExampleConfig {
//     type Id = ID64;
//     type Data = ExampleData;
//     type Registry = HashMapRegistry<Self::Data>;

//     // Use '=' to assign the concrete types
//     type UnitCategory = ExampleUnitCategory;
//     type UnitSetting = ExampleUnitSettings;  
 
//     // type Display = CommonDisplayText;
//     // type Lang = UnitedStatesLanguage;
// }

// // MODEL
// // Now you can make the model using the configuration
// pub type ExampleModel = Model<ExampleConfig>;

// // INSTANTIATION
// /// Creates a configured model for use in examples.
// pub fn create_example_model() -> ExampleModel {
//     let registry = HashMapRegistry::default();
    
//     let file_settings = ExampleUnitSettings::default();
//     let mut display_settings = ExampleUnitSettings::default();
    
//     display_settings.length = SimpleUnit::Length { 
//         unit: LengthUnit::Foot, 
//         exponent: 1 
//     };

//     let settings = UnitSystem::new(file_settings, display_settings);
//     Model::new(registry, settings)
// }
