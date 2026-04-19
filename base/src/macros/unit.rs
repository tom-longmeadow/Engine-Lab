
#[macro_export]
macro_rules! base_unit_macro {
    ($name:ident { 
        $default_variant:ident = ($default_factor:expr, $default_symbol:expr), 
        $($variant:ident = ($factor:expr, $symbol:expr)),* $(,)?
    }) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
        pub enum $name {
            #[default]
            $default_variant,
            $($variant),*
        }

        impl Unit for $name {
            // Self is the Enum type (e.g., LengthUnit)
            const DEFAULT: Self = Self::$default_variant;
            const COUNT: usize = 1 $(+ { let _ = stringify!($variant); 1 })*;

           
            fn symbol(&self) -> &'static str {
                match self {
                    Self::$default_variant => $default_symbol,
                    $(Self::$variant => $symbol),*
                }
            }

            fn all_variants() -> Vec<Self> {
                vec![Self::$default_variant, $(Self::$variant),*]
            }

            fn to_base(&self, val: f64, power: i8) -> f64 {
                val * self.conversion_factor().powi(power as i32)
            }

            fn from_base(&self, si_val: f64, power: i8) -> f64 {
                si_val / self.conversion_factor().powi(power as i32)
            }
        }

        impl $name {
            pub fn conversion_factor(&self) -> f64 {
                match self {
                    Self::$default_variant => $default_factor,
                    $(Self::$variant => $factor),*
                }
            }
        }
        
        // Bonus: Implementation for easy printing
        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.symbol())
            }
        }
    };
}

#[macro_export]
macro_rules! temperature_unit_macro {
    ($name:ident { 
        $default_variant:ident = ($default_factor:expr, $default_offset:expr, $default_symbol:expr), 
        $($variant:ident = ($factor:expr, $offset:expr, $symbol:expr)),* $(,)? 
    }) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
        pub enum $name {
            #[default]
            $default_variant,
            $($variant),*
        }

        impl Unit for $name {
            const DEFAULT: Self = Self::$default_variant;
            const COUNT: usize = 1 $(+ { let _ = stringify!($variant); 1 })*;

            fn symbol(&self) -> &'static str {
                match self {
                    Self::$default_variant => $default_symbol,
                    $(Self::$variant => $symbol),*
                }
            }
    
            fn all_variants() -> Vec<Self> {
                vec![Self::$default_variant, $(Self::$variant),*]
            }
     
            fn to_base(&self, val: f64, power: i8) -> f64 {
                let (factor, offset) = self.values();
                if power == 0 {
                    // It's a Temperature Delta (Difference)
                    val * factor
                } else {
                    // It's an Absolute Temperature
                    (val * factor) + offset
                }
            }

            fn from_base(&self, si_val: f64, power: i8) -> f64 {
                let (factor, offset) = self.values();
                if power == 0 {
                    si_val / factor
                } else {
                    (si_val - offset) / factor
                }
            }
        }

        impl $name {
            #[inline]
            fn values(&self) -> (f64, f64) {
                match self {
                    Self::$default_variant => ($default_factor, $default_offset),
                    $(Self::$variant => ($factor, $offset)),*
                }
            }
        }
    };
}

#[macro_export]
macro_rules! define_unit_system {
    ($category_enum:ident, $settings_struct:ident {
        $($field:ident : $variant:ident : $kind:ident),* $(,)?
    }) => {
        // 1. The ID Enum (e.g., ExampleUnitCategory)
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub enum $category_enum {
            $($variant),*
        }

        impl $crate::unit::UnitCategory for $category_enum {}

        // 2. The Storage Struct (e.g., ExampleUnitSetting)
        #[derive(Debug, Clone, Copy, PartialEq)]
        pub struct $settings_struct {
            // Appends "Unit" to the Kind to get the type name (Simple -> SimpleUnit)
            $(pub $field: $crate::unit::$kind),*
        }

        // 3. The Bridge Trait
        impl $crate::unit::UnitSettings<$category_enum> for $settings_struct {
            fn get_kind(&self, category: $category_enum) -> $crate::unit::UnitKind {
                match category {
                    $(
                        $category_enum::$variant => {
                            // Logic: UnitKind::Simple(self.length)
                            // Note: $kind here must match the UnitKind variant names
                            $crate::unit::UnitKind::$kind(self.$field)
                        }
                    ),*
                }
            }
        }
    };
}