 
use crate::unit::{AmountUnit, BaseUnit, CurrentUnit, LengthUnit, LuminousIntensityUnit, MassUnit, TemperatureUnit, TimeUnit, Unit};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SimpleUnit {
    Length      { unit: LengthUnit,      exponent: i8 },
    Mass        { unit: MassUnit,        exponent: i8 },
    Time        { unit: TimeUnit,        exponent: i8 },
    Temperature { unit: TemperatureUnit, exponent: i8 },
    Current     { unit: CurrentUnit,     exponent: i8 },
    Amount      { unit: AmountUnit,      exponent: i8 },
    LuminousIntensity { unit: LuminousIntensityUnit, exponent: i8 },
}

impl SimpleUnit {

    pub const fn length() -> Self {
        Self::Length { unit: LengthUnit::DEFAULT, exponent: 1 }
    }
    pub const fn area() -> Self {
        Self::Length { unit: LengthUnit::DEFAULT, exponent: 2 }
    }
    pub const fn volume() -> Self {
        Self::Length { unit: LengthUnit::DEFAULT, exponent: 3 }
    }
    pub const fn mass() -> Self {
        Self::Mass { unit: MassUnit::DEFAULT, exponent: 1 }
    }

    pub const fn frequency() -> Self {
        Self::Time { unit: TimeUnit::DEFAULT, exponent: -1 }
    }

    


    pub const fn base(&self) -> BaseUnit {
        match self {
            Self::Length      { .. } => BaseUnit::Length,
            Self::Mass        { .. } => BaseUnit::Mass,
            Self::Time        { .. } => BaseUnit::Time,
            Self::Temperature { .. } => BaseUnit::Temperature,
            Self::Current     { .. } => BaseUnit::Current,
            Self::Amount      { .. } => BaseUnit::Amount,
            Self::LuminousIntensity { .. } => BaseUnit::LuminousIntensity,
        }
    }

    pub const fn index(&self) -> usize {
        self.base() as usize
    }

    pub const fn exponent(&self) -> i8 {
        match self {
            Self::Length { exponent, .. }      => *exponent,
            Self::Mass   { exponent, .. }      => *exponent,
            Self::Time   { exponent, .. }      => *exponent,
            Self::Temperature { exponent, .. } => *exponent,
            Self::Current     { exponent, .. } => *exponent,
            Self::Amount      { exponent, .. } => *exponent,
            Self::LuminousIntensity { exponent, .. } => *exponent,
        }
    }

    pub fn to_base(&self, val: f64) -> f64 {
        match self {
            Self::Length { unit, exponent } => unit.to_base(val, *exponent),
            Self::Mass { unit, exponent } => unit.to_base(val, *exponent),
            Self::Time { unit, exponent } => unit.to_base(val, *exponent),
            // Offset logic (Celsius -> Kelvin) is handled inside the macro logic
            Self::Temperature { unit, exponent } => unit.to_base(val, *exponent),
            Self::Current { unit, exponent } => unit.to_base(val, *exponent),
            Self::Amount { unit, exponent } => unit.to_base(val, *exponent),
            Self::LuminousIntensity { unit, exponent } => unit.to_base(val, *exponent),
        }
    }

    pub fn from_base(&self, val: f64) -> f64 {
        match self {
            Self::Length { unit, exponent } => unit.from_base(val, *exponent),
            Self::Mass { unit, exponent } => unit.from_base(val, *exponent),
            Self::Time { unit, exponent } => unit.from_base(val, *exponent),
            Self::Temperature { unit, exponent } => unit.from_base(val, *exponent),
            Self::Current { unit, exponent } => unit.from_base(val, *exponent),
            Self::Amount { unit, exponent } => unit.from_base(val, *exponent),
            Self::LuminousIntensity { unit, exponent } => unit.from_base(val, *exponent),
        }
    }

    pub fn symbol(&self) -> &'static str {
        match self {
            Self::Length { unit, .. } => unit.symbol(),
            Self::Mass { unit, .. } => unit.symbol(),
            Self::Time { unit, .. } => unit.symbol(),
            Self::Temperature { unit, .. } => unit.symbol(),
            Self::Current { unit, .. } => unit.symbol(),
            Self::Amount { unit, .. } => unit.symbol(),
            Self::LuminousIntensity { unit, .. } => unit.symbol(),
        }
    }

}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CompoundUnit {
    pub components: [SimpleUnit; BaseUnit::COUNT],
}

impl CompoundUnit {

    pub const fn force() -> Self {
        Self::new()
            .with(SimpleUnit::Mass   { unit: MassUnit::DEFAULT,   exponent: 1 })
            .with(SimpleUnit::Length { unit: LengthUnit::DEFAULT, exponent: 1 })
            .with(SimpleUnit::Time   { unit: TimeUnit::DEFAULT,   exponent: -2 })
    }

    pub const fn energy() -> Self {
        Self::new()
            .with(SimpleUnit::Mass   { unit: MassUnit::DEFAULT,   exponent: 1 })
            .with(SimpleUnit::Length { unit: LengthUnit::DEFAULT, exponent: 1 })
            .with(SimpleUnit::Time   { unit: TimeUnit::DEFAULT,   exponent: -2 })
    } 
     pub const fn pressure() -> Self {
        Self::new()
            .with(SimpleUnit::Mass   { unit: MassUnit::DEFAULT,   exponent: 1 })
            .with(SimpleUnit::Length { unit: LengthUnit::DEFAULT, exponent: -1 })
            .with(SimpleUnit::Time   { unit: TimeUnit::DEFAULT,   exponent: -2 })
    }


    pub const fn new() -> Self {
        Self {
            components: [
                SimpleUnit::Length      { unit: LengthUnit::DEFAULT,      exponent: 0 },
                SimpleUnit::Mass        { unit: MassUnit::DEFAULT,        exponent: 0 },
                SimpleUnit::Time        { unit: TimeUnit::DEFAULT,        exponent: 0 },
                SimpleUnit::Temperature { unit: TemperatureUnit::DEFAULT, exponent: 0 },
                SimpleUnit::Current     { unit: CurrentUnit::DEFAULT,     exponent: 0 },
                SimpleUnit::Amount      { unit: AmountUnit::DEFAULT,      exponent: 0 },
                SimpleUnit::LuminousIntensity { unit: LuminousIntensityUnit::DEFAULT, exponent: 0 },
            ],
        }
    }

    // Change to 'const fn' and 'mut self'
    pub const fn with(mut self, component: SimpleUnit) -> Self {
        // Since component.index() is a const fn, we can use it here
        self.components[component.index()] = component;
        self
    }
}


#[derive(Debug, Clone, Copy, PartialEq)]
pub enum UnitKind {
    Simple(SimpleUnit),
    Compound(CompoundUnit),
    Temperature(TemperatureUnit),
}

impl UnitKind {

    // pub const fn unwrap_simple(self) -> SimpleUnit {
    //     match self {
    //         Self::Simple(s) => s,
    //         _ => panic!("Expected SimpleUnit"),
    //     }
    // }

    // pub const fn unwrap_compound(self) -> CompoundUnit {
    //     match self {
    //         Self::Compound(c) => c,
    //         _ => panic!("Expected CompoundUnit"),
    //     }
    // }

    // pub const fn unwrap_temperature(self) -> TemperatureUnit {
    //     match self {
    //         Self::Temperature(t) => t,
    //         _ => panic!("Expected TemperatureUnit"),
    //     }
    // }

    // pub const fn length() -> Self {
    //     Self::Simple(SimpleUnit::Length { unit: LengthUnit::DEFAULT, exponent: 1 })
    // }

    // pub const fn area() -> Self {
    //     Self::Simple(SimpleUnit::Length { unit: LengthUnit::DEFAULT, exponent: 2 })
    // }

    // pub const fn volume() -> Self {
    //     Self::Simple(SimpleUnit::Length { unit: LengthUnit::DEFAULT, exponent: 3 })
    // }

    // pub const fn mass() -> Self {
    //     Self::Simple(SimpleUnit::Mass { unit: MassUnit::DEFAULT, exponent: 1 })
    // }

    // pub const fn time() -> Self {
    //     Self::Simple(SimpleUnit::Time { unit: TimeUnit::DEFAULT, exponent: 1 })
    // }

    // pub const fn temperature() -> Self {
    //     // Using the Temperature variant you added to UnitKind
    //     Self::Temperature(TemperatureUnit::DEFAULT)
    // }

    // --- Compound Units (Force, Torque, etc.) ---

    // pub const fn force() -> Self {
    //     Self::Compound(
    //         CompoundUnit::new()
    //             .with(SimpleUnit::Mass   { unit: MassUnit::DEFAULT,   exponent: 1 })
    //             .with(SimpleUnit::Length { unit: LengthUnit::DEFAULT, exponent: 1 })
    //             .with(SimpleUnit::Time   { unit: TimeUnit::DEFAULT,   exponent: -2 })
    //     )
    // }

    // pub const fn energy() -> Self {
    //     Self::Compound(
    //         CompoundUnit::new()
    //             .with(SimpleUnit::Mass   { unit: MassUnit::DEFAULT,   exponent: 1 })
    //             .with(SimpleUnit::Length { unit: LengthUnit::DEFAULT, exponent: 2 })
    //             .with(SimpleUnit::Time   { unit: TimeUnit::DEFAULT,   exponent: -2 })
    //     )
    // }

    // pub const fn pressure() -> Self {
    //     Self::Compound(
    //         CompoundUnit::new()
    //             .with(SimpleUnit::Mass   { unit: MassUnit::DEFAULT,   exponent: 1 })
    //             .with(SimpleUnit::Length { unit: LengthUnit::DEFAULT, exponent: -1 })
    //             .with(SimpleUnit::Time   { unit: TimeUnit::DEFAULT,   exponent: -2 })
    //     )
    // }

    // pub const fn frequency() -> Self {
    //     Self::Simple(SimpleUnit::Time { unit: TimeUnit::DEFAULT, exponent: -1 })
    // }

    // pub const fn resistance() -> Self {
    //     Self::Compound(
    //         CompoundUnit::new()
    //             .with(SimpleUnit::Mass    { unit: MassUnit::DEFAULT,    exponent: 1 })
    //             .with(SimpleUnit::Length  { unit: LengthUnit::DEFAULT,  exponent: 2 })
    //             .with(SimpleUnit::Time    { unit: TimeUnit::DEFAULT,    exponent: -3 })
    //             .with(SimpleUnit::Current { unit: CurrentUnit::DEFAULT, exponent: -2 })
    //     )
    // }


    pub fn to_base(&self, val: f64) -> f64 {
        match self {
            Self::Simple(s) => s.to_base(val),
            Self::Compound(c) => {
                let mut result = val;
                for component in c.components {
                    if component.exponent() != 0 {
                        result = component.to_base(result);
                    }
                }
                result
            }
            // Explicit Temperature variant: we treat this as Absolute (power 1)
            Self::Temperature(t) => t.to_base(val, 1),
        }
    }

    pub fn from_base(&self, val: f64) -> f64 {
        match self {
            Self::Simple(s) => s.from_base(val),
            Self::Compound(c) => {
                let mut result = val;
                for component in c.components.iter().rev() {
                    if component.exponent() != 0 {
                        result = component.from_base(result);
                    }
                }
                result
            }
            // Explicit Temperature variant: we treat this as Absolute (power 1)
            Self::Temperature(t) => t.from_base(val, 1),
        }
    }

    pub fn symbol(&self) -> String {
        match self {
            Self::Simple(s) => {
                let exp = s.exponent();
                if exp == 1 {
                    s.symbol().to_string()
                } else {
                    format!("{}^{}", s.symbol(), exp)
                }
            }
            Self::Temperature(t) => t.symbol().to_string(),
            Self::Compound(c) => {
                let mut parts = Vec::new();
                for comp in c.components {
                    let exp = comp.exponent();
                    if exp != 0 {
                        if exp == 1 {
                            parts.push(comp.symbol().to_string());
                        } else {
                            parts.push(format!("{}^{}", comp.symbol(), exp));
                        }
                    }
                }
                parts.join("·") // Or "*" depending on your preference
            }
        }
    }
 

}

// #[derive(Debug, Clone, Copy, PartialEq)]
// pub struct SimpleUnit {
//     pub base: BaseUnit,
//     pub exponent: i8,
// }

// impl SimpleUnit {
//     pub const fn new(base: BaseUnit, exponent: i8) -> Self {
//         Self {
//             base,
//             exponent
//         }
//     }
// }


// #[derive(Debug, Clone, Copy, PartialEq)]
// pub struct CompoundUnit {
//     pub dimensions: [i8; BaseUnit::COUNT]
// }

// impl CompoundUnit {
//     /// Creates a new CompoundUnit with all exponents set to 0.
//     pub const fn new() -> Self {
//         Self {
//             dimensions: [0i8; BaseUnit::COUNT],
//         }
//     }

//     /// Sets the exponent for a specific base unit and returns the unit.
//     pub const fn with(mut self, base: BaseUnit, exponent: i8) -> Self {
//         self.dimensions[base.index()] = exponent;
//         self
//     }
// }

// pub enum UnitKind {
//     Simple(SimpleUnit),
//     Compound(CompoundUnit),
//     Temperature(TemperatureUnit),
// }

