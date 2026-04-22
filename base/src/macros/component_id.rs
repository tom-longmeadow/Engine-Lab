

#[macro_export]
macro_rules! component_id_primitive_macro {
    ($($t:ty),*) => {
        $(
            impl ComponentId for $t {
                fn invalid() -> Self { 0 }
            }
        )*
    };
}

#[macro_export]
macro_rules! component_id_macro {
    ($name:ident, $type:ty) => {

        #[repr(transparent)]
        #[derive(
            ::derive_more::From, 
            ::derive_more::Into, 
            ::derive_more::AsRef, 
            ::derive_more::Display,
            ::derive_more::FromStr, // 👈 ADD THIS HERE
            Copy, Clone, Eq, PartialEq, Hash, Debug
        )]
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

