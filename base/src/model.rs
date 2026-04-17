// 1. Declare the sub-modules (files/folders inside src/model/)
pub mod component;
pub mod registry;
pub mod error;

// 3. Handle feature-gated modules
#[cfg(feature = "testing")]
pub mod test_model;

pub use component::*;
pub use registry::*;
pub use error::*;

use crate::{
    language::{DisplayText, Language}, 
    unit::{UnitCategory, UnitSetting, UnitSettings}
};

 
pub trait ModelConfig: 'static {
    // Data Types
    type Id: ComponentId;
    type Data: ComponentData;
    
    // Storage Type (Linked to Data Types)
    type Registry: ComponentRegistry<Id = Self::Id, Data = Self::Data>;

    // Unit Types
    type Category: UnitCategory;
    type Setting: UnitSetting<Self::Category>;

    // Languages
    type Display: DisplayText; 
    type Lang: Language;
    //type Translator: TranslationProvider<Self::Lang>;
}
 
pub struct Model<C> 
where 
    C: ModelConfig
{
    pub registry: C::Registry, 
    pub settings: UnitSettings<C>,
}

impl<C> Model<C>  
where 
    C: ModelConfig,
    C::Data: ComponentData,                  
    <C::Data as ComponentData>::Kind: ComponentKind<Id = C::Id>, // Link Id types
{

    pub fn new(registry: C::Registry, settings: UnitSettings<C>) -> Self {
        Self { registry, settings }
    }

    pub fn insert(&mut self, id: C::Id, data: C::Data) -> Result<(), ModelError<C::Data, C::Id>> {
        let kind = data.kind();
        if id.is_invalid() {
            return Err(ModelError::InvalidId(id, kind));
        }  
        if self.registry.contains(&id, kind) {
            return Err(ModelError::AlreadyExists(id, kind));
        }
        self.registry.insert(id, data);
        Ok(())
    }

    pub fn update(&mut self, id: C::Id, data: C::Data) -> Result<(), ModelError<C::Data, C::Id>> {
        let kind = data.kind();  
        if id.is_invalid() {
            return Err(ModelError::InvalidId(id, kind));
        }  
        if !self.registry.contains(&id, kind) {
            return Err(ModelError::NotFound(id, kind)); 
        }
        self.registry.insert(id, data);
        Ok(())
    }

    pub fn get(&self, id: C::Id, kind: <C::Data as ComponentData>::Kind) -> Result<&C::Data, ModelError<C::Data, C::Id>> {
        if id.is_invalid() {
            return Err(ModelError::InvalidId(id, kind));
        }
        self.registry.get(&id, kind).ok_or_else(|| ModelError::NotFound(id, kind))
    }

    pub fn get_mut(&mut self, id: C::Id, kind: <C::Data as ComponentData>::Kind) -> Result<&mut C::Data, ModelError<C::Data, C::Id>> {
        if id.is_invalid() {
            return Err(ModelError::InvalidId(id, kind));
        }
        self.registry.get_mut(&id, kind).ok_or_else(|| ModelError::NotFound(id, kind))
    }

    pub fn get_clone(&self, id: C::Id, kind: <C::Data as ComponentData>::Kind) -> Result<C::Data, ModelError<C::Data, C::Id>> {
        self.get(id, kind).map(|data| data.clone())
    }

    pub fn delete(&mut self, id: C::Id, kind: <C::Data as ComponentData>::Kind) -> Result<C::Data, ModelError<C::Data, C::Id>> {
        if id.is_invalid() {
            return Err(ModelError::InvalidId(id, kind));
        }
        self.registry.remove(&id, kind).ok_or_else(|| ModelError::NotFound(id, kind))
    }

    // Component wrappers
    pub fn insert_comp(&mut self, comp: Component<C::Data>) -> Result<(), ModelError<C::Data, C::Id>> {
        self.insert(comp.id, comp.data)
    }

    pub fn update_comp(&mut self, comp: Component<C::Data>) -> Result<(), ModelError<C::Data, C::Id>> {
        self.update(comp.id, comp.data)
    }

    // Iterators
    pub fn components(&self) -> impl Iterator<Item = &C::Data> {
        self.registry.values()
    }

    pub fn components_by_kind(&self, kind: <C::Data as ComponentData>::Kind) -> impl Iterator<Item = &C::Data> {
        self.registry.values_by_kind(kind)
    }

    pub fn components_mut(&mut self) -> impl Iterator<Item = &mut C::Data> {
        self.registry.values_mut()
    }

    pub fn components_mut_by_kind(&mut self, kind: <C::Data as ComponentData>::Kind) -> impl Iterator<Item = &mut C::Data> {
        self.registry.values_mut_by_kind(kind)
    }

   

    // pub fn insert(&mut self, id: C::Id, data: C::Data) -> Result<(), ModelError<C::Data, C::Id>> {
    //     let kind = data.kind();
    //     if id.is_invalid() {
    //         return Err(ModelError::InvalidId(id, kind));
    //     }  
    //     if self.registry.contains(&id, kind) {
    //         return Err(ModelError::AlreadyExists(id, kind));
    //     }
    //     self.registry.insert(id, data);
    //     Ok(())
    // }

   

    // pub fn update(&mut self, id: &C::Id, data: C::Data) -> Result<(), ModelError<C::Data, C::Id>> {
    //     let kind = data.kind();  
    //     if id.is_invalid() {
    //         return Err(ModelError::InvalidId(id.clone(), kind));
    //     }  
    //     if !self.registry.contains(id, kind) {
    //         return Err(ModelError::NotFound(id.clone(), kind)); 
    //     }
    //     self.registry.insert(id.clone(), data);
    //     Ok(())
    // }

   

    // pub fn get(&self, id: &C::Id, kind: <C::Data as ComponentData>::Kind) -> Result<&C::Data, ModelError<C::Data, C::Id>> {
    //     if id.is_invalid() {
    //         return Err(ModelError::InvalidId(id.clone(), kind));
    //     }
    //     self.registry.get(id, kind).ok_or_else(|| ModelError::NotFound(id.clone(), kind))
    // }

     

    // pub fn get_mut(&mut self, id: &C::Id, kind: <C::Data as ComponentData>::Kind) -> Result<&mut C::Data, ModelError<C::Data, C::Id>> {
    //     if id.is_invalid() {
    //         return Err(ModelError::InvalidId(id.clone(), kind));
    //     }
    //     self.registry.get_mut(id, kind).ok_or_else(|| ModelError::NotFound(id.clone(), kind))
    // }

    
    // pub fn get_clone(&self, id: &C::Id, kind: <C::Data as ComponentData>::Kind) -> Result<C::Data, ModelError<C::Data, C::Id>> {
    //     self.get(id, kind).map(|data| data.clone())
    // }
    

    // pub fn delete(&mut self, id: &C::Id, kind: <C::Data as ComponentData>::Kind) -> Result<C::Data, ModelError<C::Data, C::Id>> {
    //     if id.is_invalid() {
    //         return Err(ModelError::InvalidId(id.clone(), kind));
    //     }
    //     self.registry.remove(id, kind).ok_or_else(|| ModelError::NotFound(id.clone(), kind))
    // }

   
    // // component 
    // pub fn insert_comp(&mut self, comp: Component<C::Data>) -> Result<(), ModelError<C::Data, C::Id>> {
    //     self.insert(comp.id, comp.data)
    // }

    // pub fn update_comp(&mut self, comp: Component<C::Data>) -> Result<(), ModelError<C::Data, C::Id>> {
    //     self.update(&comp.id, comp.data)
    // }

    

    // // Iterators
    // pub fn components(&self) -> impl Iterator<Item = &C::Data> {
    //     self.registry.values()
    // }

    // pub fn components_by_kind(&self, kind: <C::Data as ComponentData>::Kind) -> impl Iterator<Item = &C::Data> {
    //     self.registry.values_by_kind(kind)
    // }

    // pub fn components_mut(&mut self) -> impl Iterator<Item = &mut C::Data> {
    //     self.registry.values_mut()
    // }

    // pub fn components_mut_by_kind(&mut self, kind: <C::Data as ComponentData>::Kind) -> impl Iterator<Item = &mut C::Data> {
    //     self.registry.values_mut_by_kind(kind)
    // }
}
 