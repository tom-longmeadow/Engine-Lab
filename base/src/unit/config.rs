 

use super::{
    UnitCategory,
    UnitSettings
};


pub trait UnitConfig: 'static {
    // Unit Categories
    type UnitCategory: UnitCategory;
    /// The Struct "Storage" that holds the actual SimpleUnits/CompoundUnits
    type UnitSetting: UnitSettings<Self::UnitCategory>;
}