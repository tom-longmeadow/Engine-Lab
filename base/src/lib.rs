#[macro_use]
pub mod macros;  

pub mod model;
pub mod property;
pub mod unit;

// Re-exports: Make these available at the crate root (e.g., my_crate::Registry)
pub use crate::model::{
    component::*,
    registry::*,
    error::*,
    Model,
};

pub mod prelude {
    // Re-export the traits so methods are available
    //pub use crate::unit::Unit; 
    
    // Re-export the core items
    pub use crate::model::{
        component::*,
        registry::*,
        error::*,
        Model,
    };
    
    // Often useful to include the macros in the prelude
    pub use crate::{enum_macro, base_unit_macro};
}

