
use std::collections::HashMap;
use base::model::{ComponentData, ComponentKey, ComponentKind, ComponentRegistry};


/// A registry that uses a single hashmap to store all component kinds.
pub struct HashMapRegistry<D> 
where 
    D: ComponentData,
{
    // The key is now derived entirely from the Data's Kind
    items: HashMap<ComponentKey<D::Kind>, D>,
}

impl<D> Default for HashMapRegistry<D> 
where 
    D: ComponentData 
{
    fn default() -> Self {
        Self {
            items: HashMap::new(),
        }
    }
}

impl<D> ComponentRegistry for HashMapRegistry<D>
where
    D: ComponentData,
    <D as ComponentData>::Kind: ComponentKind,
{
    type Id = <<D as ComponentData>::Kind as ComponentKind>::Id;
    type Data = D;

    fn insert(&mut self, id: Self::Id, data: Self::Data) -> Option<Self::Data> {
        let key = ComponentKey { id, kind: data.kind() };
        self.items.insert(key, data)
    }

    fn remove(&mut self, id: &Self::Id, kind: D::Kind) -> Option<Self::Data> {
        let key = ComponentKey { id: *id, kind };
        self.items.remove(&key)
    }

    fn get(&self, id: &Self::Id, kind: D::Kind) -> Option<&Self::Data> {
        let key = ComponentKey { id: *id, kind };
        self.items.get(&key)
    }

    fn get_mut(&mut self, id: &Self::Id, kind: D::Kind) -> Option<&mut Self::Data> {
        let key = ComponentKey { id: *id, kind };
        self.items.get_mut(&key)
    }

    fn contains(&self, id: &Self::Id, kind: D::Kind) -> bool {
        let key = ComponentKey { id: *id, kind };
        self.items.contains_key(&key)
    }

    // Explicit implementations for the iterators
    fn values(&self) -> impl Iterator<Item = &Self::Data> {
        self.items.values()
    }

    fn values_mut(&mut self) -> impl Iterator<Item = &mut Self::Data> {
        self.items.values_mut()
    }

    // Overriding the defaults for better efficiency (filtering the iterator directly)
    fn values_by_kind(&self, kind: D::Kind) -> impl Iterator<Item = &Self::Data> {
        self.items.iter()
            .filter(move |(k, _)| k.kind == kind)
            .map(|(_, v)| v)
    }

    fn values_mut_by_kind(&mut self, kind: D::Kind) -> impl Iterator<Item = &mut Self::Data> {
        self.items.iter_mut()
            .filter(move |(k, _)| k.kind == kind)
            .map(|(_, v)| v)
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    
    // Test the low-level storage
    base::test_registry!(HashMapRegistry);
    
    // Test the high-level API
    base::test_model!(HashMapRegistry);
}



// /// An example of how to create a registry that uses a hashmap
// pub struct HashMapRegistry<I, D> 
// where 
//     I: ComponentId, 
//     D: ComponentData 
// {
//     items: HashMap<ComponentKey<I, D::Kind>, D>,
// }

// impl<I, D> HashMapRegistry<I, D> 
// where 
//     I: ComponentId, 
//     D: ComponentData 
// {
//     pub fn new() -> Self {
//         Self::default()
//     }
// }

// impl<I, D> Default for HashMapRegistry<I, D> 
// where I: ComponentId, D: ComponentData 
// {
//     fn default() -> Self {
//         Self {
//             items: HashMap::new(),
//         }
//     }
// }

// impl<I, D> ComponentRegistry for HashMapRegistry<I, D>
// where
//     I: ComponentId,
//     D: ComponentData,
// {
//     type Id = I;
//     type Data = D;

//     fn insert(&mut self, id: Self::Id, data: Self::Data) -> Option<Self::Data> {
//         let key = ComponentKey { id, kind: data.kind() };
//         self.items.insert(key, data)
//     }

//     fn remove(&mut self, id: &Self::Id, kind: D::Kind) -> Option<Self::Data> {
//         let key = ComponentKey { id: id.clone(), kind };
//         self.items.remove(&key)
//     }

//     fn get(&self, id: &Self::Id, kind: D::Kind) -> Option<&Self::Data> {
//         let key = ComponentKey { id: id.clone(), kind };
//         self.items.get(&key)
//     }

//     fn get_mut(&mut self, id: &Self::Id, kind: D::Kind) -> Option<&mut Self::Data> {
//         let key = ComponentKey { id: id.clone(), kind };
//         self.items.get_mut(&key)
//     }

//     fn contains(&self, id: &Self::Id, kind: D::Kind) -> bool {
//         let key = ComponentKey { id: id.clone(), kind };
//         self.items.contains_key(&key)
//     }

//     fn values(&self) -> impl Iterator<Item = &Self::Data> {
//         self.items.values()
//     }

//     fn values_mut(&mut self) -> impl Iterator<Item = &mut Self::Data> {
//         self.items.values_mut()
//     }
// }