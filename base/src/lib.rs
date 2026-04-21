pub use derive_more; 

#[macro_use]
pub mod macros;  

pub mod model;
pub mod property;
pub mod unit;
pub mod language;

pub mod prelude {

    pub use crate::language::*;

    pub use crate::unit::{
        base_unit::*,
        category::*, 
        kind::*,
        simple::*,
        compound::*,
        settings::*,
        UnitSystem,
    };
    
    pub use crate::property::{
        propertied::*,
        node::*,
        error::*,
        value::*,
        PropertyConfig,
        Property,
    };

    pub use crate::model::{
        component::*,
        registry::*,
        error::*,
        ModelConfig,
        Model,
    };
    
    // Often useful to include the macros in the prelude
    pub use crate::{
        enum_macro, 
        enum_index_macro,
        base_unit_macro,
        temperature_unit_macro,
        component_id_macro,
        component_id_primitive_macro
    };
}

