
use super::{PropertyConfig, Property};

pub enum PropertyNode<C: PropertyConfig> {
    Leaf(Property<C>),
    Group {
        name: C::Display,
        children: Vec<PropertyNode<C>>,
    },
}

 