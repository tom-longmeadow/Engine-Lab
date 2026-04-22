
 use crate::{
    component_id_macro, 
    component_id_primitive_macro, 
    prelude::{
        DisplayText, Propertied, Property, PropertyConfig, PropertyError, PropertyNode, PropertyValue
    }
};

 pub trait ComponentId: Copy + Eq + std::hash::Hash + std::fmt::Debug + std::fmt::Display + std::str::FromStr {
    /// Returns the "null" or "free" version of this ID.
    fn invalid() -> Self; 

    /// Checks if this ID is the "null" state.
    fn is_invalid(&self) -> bool {
        *self == Self::invalid()
    }

    fn to_option(self) -> Option<Self> {
        if self.is_invalid() { None } else { Some(self) }
    }
}

// Create the ability to use unsigned integers as an Id
component_id_primitive_macro!(u32, u64, u128, usize);

component_id_macro!(IDu, usize);
component_id_macro!(ID128, u128);
component_id_macro!(ID64, u64);
component_id_macro!(ID32, u32);

pub trait ComponentKind: Copy + Eq + std::hash::Hash + std::fmt::Debug {
    type Id: ComponentId;  
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct ComponentKey<K: ComponentKind> {
    pub id: K::Id, // 🔒 This forces them to match perfectly!
    pub kind: K,
}

pub trait ComponentData<C: PropertyConfig>: Clone + Propertied<C> {
    type Kind: ComponentKind; 
    fn kind(&self) -> Self::Kind;
}

 
pub struct Component<C: PropertyConfig, D: ComponentData<C>> {  
    pub id: <<D as ComponentData<C>>::Kind as ComponentKind>::Id,
    pub data: D,
}

impl<C: PropertyConfig, D: ComponentData<C>> Component<C, D> {
 
    pub fn id(&self) -> <<D as ComponentData<C>>::Kind as ComponentKind>::Id {
        self.id
    }
    
    pub fn data(&self) -> &D { 
        &self.data 
    }
    
    pub fn kind(&self) -> D::Kind {
        self.data.kind()
    }

    pub fn key(&self) -> ComponentKey<D::Kind> {
        ComponentKey {
            id: self.id,
            kind: self.kind(),
        }
    }
}

impl<C: PropertyConfig + Sized, D: ComponentData<C>> Component<C, D> {
    pub const ID_KEY: u64 = Property::<C>::hash_key("ID");
}

impl<C: PropertyConfig, D: ComponentData<C>> Propertied<C> for Component<C, D> {
  

   fn get_template() -> Vec<PropertyNode<C>> {
        type KindId<C, D> = <<D as ComponentData<C>>::Kind as ComponentKind>::Id;
        
        let id: KindId<C, D> = KindId::<C, D>::invalid();

        let mut template = vec![ 
            PropertyNode::text(DisplayText::ID.into(), id.to_string()) 
        ]; 
         
        template.extend(D::get_template());
        template
    }

    fn get_value(&self, prop: &Property<C>) -> PropertyValue {
      
        if prop.key == Component::<C, D>::ID_KEY {
            let id: String = self.id.to_string(); 
            return PropertyValue::Text(id);
        }
         
        self.data.get_value(prop)
    }

    fn set_value(&mut self, prop: &Property<C>, value: PropertyValue) -> Result<(), PropertyError> {
    
        if prop.key == Component::<C, D>::ID_KEY { 
            self.id = prop.parse_as(value)?;
            return Ok(());
        }

        self.data.set_value(prop, value)
    }
}
 
 


// pub trait ComponentId: Copy + Eq + std::hash::Hash + std::fmt::Debug {
//     /// Returns the "null" or "free" version of this ID.
//     /// UUID nil or for u64, 0.
//     fn invalid() -> Self; 

//     /// Checks if this ID is the "null" state.
//     fn is_invalid(&self) -> bool {
//         *self == Self::invalid()
//     }

//     fn to_option(self) -> Option<Self> {
//         if self.is_invalid() { None } else { Some(self) }
//     }
// }

// // Create the ability to use unsigned integers as an Id
// component_id_primitive_macro!(u8, u16, u32, u64, u128, usize);

// // Making a component ID of type name = IDu, primitive type = usize
// component_id_macro!(IDu, usize);

// // Making a component ID of type name = ID64, primitive type = u64
// component_id_macro!(ID64, u64);


// /// Should be a component type enum.  For instance enum StructuralType {Joint, Member}
// pub trait ComponentKind: Copy + Eq + std::hash::Hash + std::fmt::Debug {
//     type Id: ComponentId; // The ID type is "locked" to the Kind
// }
 

// /// The key for storing components of different kinds in the same collection
// /// unique with (id, type)
// #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
// pub struct ComponentKey<K: ComponentKind> {
//     pub id: K::Id,
//     pub kind: K,
// }
 
// pub trait ComponentData: Clone  {
//     type Kind: ComponentKind; 
//     fn kind(&self) -> Self::Kind;
// }

// /// A component in the model
// pub struct Component<D: ComponentData> {
//     pub id: <<D as ComponentData>::Kind as ComponentKind>::Id,
//     pub data: D,
// }

// impl<D: ComponentData> Component<D> {
    
//     pub fn id(&self) -> <<D as ComponentData>::Kind as ComponentKind>::Id {
//         self.id
//     }
//     pub fn data(&self) -> &D { &self.data }
    
//     pub fn kind(&self) -> D::Kind {
//         self.data.kind()
//     }

//     pub fn key(&self) -> ComponentKey<D::Kind> {
//         ComponentKey {
//             id: self.id,
//             kind: self.kind(),
//         }
//     }
// }

  