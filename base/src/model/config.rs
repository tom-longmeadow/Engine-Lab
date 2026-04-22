use super::{ComponentData, ComponentId, ComponentRegistry};

use crate::{
    model::ComponentKind, prelude::PropertyConfig
};

pub trait ModelConfig: PropertyConfig + Sized {
    type Data: ComponentData<Self>; 
    
    type Registry: ComponentRegistry<
        Self, 
        Data = Self::Data, 
        Id = <<Self::Data as ComponentData<Self>>::Kind as ComponentKind>::Id 
    >;
}


// pub trait ModelConfig: PropertyConfig + Sized {
//     type Data: ComponentData<Self>;
//     type Kind: ComponentKind<Self::Data>;
//     type Id: ComponentId<Self::Kind>;

//     // The registry pulls it directly from the Data's Kind mapping
//     type Registry: ComponentRegistry<
//         Self, 
//         Id = Self::Id, 
//         Data = Self::Data
//     >;
// }


// pub trait ModelConfig: PropertyConfig + Sized {
    
//     type Data: ComponentData<Self>;
//     type Id1 = <<Self::Data as ComponentData<C>>::Kind as ComponentKind>::Id;
//     type Id2 = <<<Self::Data as ComponentData<C>>::Kind as ComponentKind>::Id;
//     //type Id: ComponentId = <<<<Self::Data as ComponentData<Self>>::Kind as ComponentKind>::Id>;
     
//     type Registry: ComponentRegistry<Self, Id = Self::Id, Data = Self::Data>;
// }