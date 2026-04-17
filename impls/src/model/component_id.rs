use base::model::{ComponentId};

/// macro to generate ids
macro_rules! define_id_type {
    ($name:ident, $type:ty) => {
        #[repr(transparent)]
        // Use separate lines or standard commas for derive_more macros
        #[derive(derive_more::From, derive_more::Into, derive_more::AsRef, derive_more::Display)]
        #[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
        pub struct $name(pub $type);

        impl $name {
            pub fn new(val: $type) -> Option<Self> {
                let id = Self(val);
                if id.is_invalid() { None } else { Some(id) }
            }
        }

        impl ComponentId for $name {
            fn invalid() -> Self {
                Self(0)
            }
        }

        impl Default for $name {
            fn default() -> Self {
                Self::invalid()
            }
        }

        impl PartialEq<$type> for $name {
            fn eq(&self, other: &$type) -> bool {
                self.0 == *other
            }
        }

        impl std::ops::Deref for $name {
            type Target = $type;
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }
    };
}


define_id_type!(Id32, u32);
define_id_type!(Id64, u64);
define_id_type!(IdSz, usize);
 

 




// #[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
// pub struct Idu32(pub u32);
// impl ComponentId for Idu32 {} 

// #[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
// pub struct IdStr(pub String);  
// impl ComponentId for IdStr {}  
