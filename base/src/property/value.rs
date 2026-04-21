 
 
pub enum PropertyValue {
    Text(String),
    Number(f64), // Used for both unitless and unit-based math
    Percent(f64),
    Integer(i64),
    Boolean(bool),
}

 
 