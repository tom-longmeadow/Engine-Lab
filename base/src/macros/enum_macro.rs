
#[macro_export]
macro_rules! enum_macro {
    ($name:ident { $($variant:ident),* $(,)? }) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub enum $name {
            $($variant),*
        }

        impl $name {
            pub const ALL: &[Self] = &[
                $(Self::$variant),*
            ];

            pub const COUNT: usize = Self::ALL.len();
        }
    };
}