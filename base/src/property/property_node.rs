
use super::{ModelConfig, Property};

pub enum PropertyNode<C: ModelConfig> {
    Leaf(Property<C>),
    Group {
        name: C::Display,
        children: Vec<PropertyNode<C>>,
    },
}

 