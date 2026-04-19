use crate::unit::{UnitCategory, UnitKind};



// Need to track the current unit for each category
pub trait UnitSettings<C: UnitCategory>: Copy + Clone + PartialEq {
    /// Maps a Category ID to its actual physical definition and selection
    fn get_kind(&self, category: C) -> UnitKind;
}
