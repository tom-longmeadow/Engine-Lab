
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
            // This now matches the trait definition
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

            fn to_si(&self, val: f64, power: i8) -> f64 {
                val * self.conversion_factor().powi(power as i32)
            }

            fn from_si(&self, si_val: f64, power: i8) -> f64 {
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
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
        pub enum $name {
            #[default]
            $default_variant,
            $($variant),*
        }

        impl Unit for $name {

            const COUNT: usize = 1 $(+ { let _ = stringify!($variant); 1 })*;

            fn symbol(&self) -> &'static str {
                self.symbol()
            }
    
            fn all_variants() -> Vec<Self> {
                vec![Self::$default_variant, $(Self::$variant),*]
            }
     
            fn to_si(&self, val: f64, _power: i8) -> f64 {
                // Absolute temperature ignores power logic
                self.to_base(val)
            }
            fn from_si(&self, si_val: f64, _power: i8) -> f64 {
                self.from_base(si_val)
            }
        }

        impl $name {
            pub fn to_base(&self, value: f64) -> f64 {
                match self {
                    Self::$default_variant => (value * $default_factor) + $default_offset,
                    $(Self::$variant => (value * $factor) + $offset),*
                }
            }

            pub fn from_base(&self, kelvin: f64) -> f64 {
                match self {
                    Self::$default_variant => (kelvin - $default_offset) / $default_factor,
                    $(Self::$variant => (kelvin - $offset) / $factor),*
                }
            }

            pub fn symbol(&self) -> &'static str {
                match self {
                    Self::$default_variant => $default_symbol,
                    $(Self::$variant => $symbol),*
                }
            }
        }
    };
}