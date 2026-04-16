

#[derive(Debug, PartialEq)]
pub enum PropertyError {
    /// The property name doesn't exist on this object
    NotFound(String),
    /// The UI sent a Number, but the property is a Boolean
    InvalidType { expected: String, received: String },
    /// The value is the right type, but out of bounds (e.g., -5.0 for voltage)
    InvalidValue(String),
    /// The property is read-only
    ReadOnly(String),
}

impl std::fmt::Display for PropertyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotFound(name) => write!(f, "Property '{}' not found", name),
            Self::InvalidType { expected, received } => {
                write!(f, "Type mismatch: expected {}, got {}", expected, received)
            }
            Self::InvalidValue(msg) => write!(f, "Invalid value: {}", msg),
            Self::ReadOnly(name) => write!(f, "Property '{}' is read-only", name),
        }
    }
}


/// The type of property
pub enum PropertyValue {
    Text(String),
    Number(f64),
    Integer(i64),
    Boolean(bool),
    // For Enums: contains the current selection and a list of all possible options
    Choice { 
        selected: String, 
        options: Vec<String> 
    },

    /// supports a tree of properties
    Group(Vec<Property>), 
}

/// Represents each property
pub struct Property {
    pub name: String,
    pub value: PropertyValue,
    pub unit: Option<String>,
    pub description: Option<String>,
    pub is_readonly: bool,
}

/// The interface for getting and setting properties
pub trait PropertyInterface {
    /// Returns a snapshot of all properties for UI display/spreadsheets.
    fn get_properties(&self) -> Vec<Property>;
    
    /// Updates a specific property. 
    /// Returns a PropertyError if the name is unknown, the type is wrong, 
    /// or the value is out of bounds.
    fn set_property(&mut self, key: &str, value: PropertyValue) -> Result<(), PropertyError>;
}
 
// pub struct LimitConfig {
//     pub max_rpm: f64,
//     pub max_temp: f64,
// }

// // In your Motor's PropertyInterface implementation:
// fn get_properties(&self) -> Vec<Property> {
//     vec![
//         Property::new("id", "ID", PropertyValue::Text(self.id.clone())),
        
//         // Nesting the LimitConfig properties under a Group
//         Property::new("limits", "Safety Limits", PropertyValue::Group(vec![
//             Property::new("max_rpm", "Max RPM", PropertyValue::Number(self.limits.max_rpm)),
//             Property::new("max_temp", "Max Temperature", PropertyValue::Number(self.limits.max_temp)),
//         ])),
//     ]
// }