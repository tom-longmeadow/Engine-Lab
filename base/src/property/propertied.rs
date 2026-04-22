  
use super::{
    PropertyConfig, PropertyValue, Property, PropertyError, PropertyNode
};

pub trait Propertied<C: PropertyConfig> {
    
    fn get_template() -> Vec<PropertyNode<C>> where Self: Sized;  
    fn get_value(&self, prop: &Property<C>) -> PropertyValue;  
    fn set_value(&mut self, prop: &Property<C>, value: PropertyValue) -> Result<(), PropertyError>;
}



 