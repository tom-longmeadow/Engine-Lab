use super::{ComponentId, ComponentData};

/// Represents the registry for components, where the 
/// component is stored as ID, Data, rather than the component struct
pub trait ComponentRegistry {
    type Id: ComponentId;
    type Data: ComponentData;

    /// Inserts the data for the component and returns the old data if the component existed.
    fn insert(&mut self, id: Self::Id, data: Self::Data) -> Option<Self::Data>;

    /// Deletes the data for the component and returns the data.
    fn remove(&mut self, id: &Self::Id, kind: <Self::Data as ComponentData>::Kind) -> Option<Self::Data>;

    /// ReadOnly reference.
    fn get(&self, id: &Self::Id, kind: <Self::Data as ComponentData>::Kind) -> Option<&Self::Data>;
    
    /// Mutable reference
    fn get_mut(&mut self, id: &Self::Id, kind: <Self::Data as ComponentData>::Kind) -> Option<&mut Self::Data>;

    fn contains(&self, id: &Self::Id, kind: <Self::Data as ComponentData>::Kind) -> bool;

    /// Get all components as readonly
    fn values(&self) -> impl Iterator<Item = &Self::Data>;

    /// Get all the components as mutable
    fn values_mut(&mut self) -> impl Iterator<Item = &mut Self::Data>;

    
    /// To loop over components by kind
    fn values_by_kind(&self, kind: <Self::Data as ComponentData>::Kind) -> impl Iterator<Item = &Self::Data> {
        self.values().filter(move |d| d.kind() == kind)
    }

    /// To loop over components by kind
    fn values_mut_by_kind(&mut self, kind: <Self::Data as ComponentData>::Kind) -> impl Iterator<Item = &mut Self::Data> {
        self.values_mut().filter(move |d| d.kind() == kind)
    }
    
}
  