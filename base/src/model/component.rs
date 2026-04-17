 
pub trait ComponentId: Copy + Eq + std::hash::Hash + std::fmt::Debug {
    /// Returns the "null" or "free" version of this ID.
    /// UUID nil or for u64, 0.
    fn invalid() -> Self; 

    /// Checks if this ID is the "null" state.
    fn is_invalid(&self) -> bool {
        *self == Self::invalid()
    }

    fn to_option(self) -> Option<Self> {
        if self.is_invalid() { None } else { Some(self) }
    }
}


macro_rules! impl_component_id_primitive {
    ($($t:ty),*) => {
        $(
            impl ComponentId for $t {
                fn invalid() -> Self { 0 }
            }
        )*
    };
}

// Create the ability to use unsigned integers as an Id
impl_component_id_primitive!(u8, u16, u32, u64, u128, usize);


/// Should be a component type enum.  For instance enum StructuralType {Joint, Member}
pub trait ComponentKind: Copy + Eq + std::hash::Hash + std::fmt::Debug {
    type Id: ComponentId; // The ID type is "locked" to the Kind
}

/// The key for storing components of different kinds in the same collection
/// unique with (id, type)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct ComponentKey<K: ComponentKind> {
    pub id: K::Id,
    pub kind: K,
}
 
pub trait ComponentData: Clone  {
    type Kind: ComponentKind; 
    fn kind(&self) -> Self::Kind;
}

/// A component in the model
pub struct Component<D: ComponentData> {
    pub id: <<D as ComponentData>::Kind as ComponentKind>::Id,
    pub data: D,
}

impl<D: ComponentData> Component<D> {
    
    pub fn id(&self) -> <<D as ComponentData>::Kind as ComponentKind>::Id {
        self.id
    }
    pub fn data(&self) -> &D { &self.data }
    
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

  