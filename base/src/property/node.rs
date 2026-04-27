use crate::prelude::{PropertyConfig, PropertyName, PropertySchema, PropertyValueKind};

 

#[derive(Debug, Clone)]
pub enum PropertyNode<C: PropertyConfig> { 
    Group {
        name: PropertyName<C>,
        children: Vec<PropertyNode<C>>,
    }, 
    Leaf(PropertySchema<C>),
}

impl<C: PropertyConfig> PropertyNode<C> {
     
    pub fn name(&self) -> &PropertyName<C> {
        match self {
            Self::Group { name, .. } => name,
            Self::Leaf(schema) => &schema.name,
        }
    }

    /// Helper to get the pre-computed hash key of this node
    pub fn key(&self) -> u64 {
        match self {
            Self::Group { name, .. } => { 
                PropertySchema::<C>::hash_key(&name.to_string())
            }
            Self::Leaf(schema) => schema.key,
        }
    }

    pub fn new(
        name: C::Display,
        kind: PropertyValueKind,
        unit: Option<C::UnitCategory>,
        key: u64,
    ) -> Self {
        Self::Leaf(PropertySchema::new(name, kind, unit, key))
    }

    pub fn new_readonly(
        name: C::Display,
        kind: PropertyValueKind,
        unit: Option<C::UnitCategory>,
        key: u64,
    ) -> Self {
        Self::Leaf(PropertySchema::new_readonly(name, kind, unit, key))
    }

    pub fn new_number(
        name: C::Display,
        unit: C::UnitCategory,
        key: u64,
    ) -> Self {
        Self::Leaf(PropertySchema::new_number(name, unit, key))
    }

    pub fn new_id(
        name: C::Display,
        key: u64,
    ) -> Self {
        Self::Leaf(PropertySchema::new_id(name, key))
    }

    pub fn new_id_readonly(
        name: C::Display,
        key: u64,
    ) -> Self {
        Self::Leaf(PropertySchema::new_id_readonly(name, key))
    }

    pub fn new_group(name: C::Display, children: Vec<PropertyNode<C>>) -> Self {
        Self::Group {
            name: PropertyName::new(name),
            children,
        }
    }
}
 
 
 

 


 
    