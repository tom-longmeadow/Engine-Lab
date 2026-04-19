


// #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
// pub enum StructuralUnitCategory{
//     Length, 
//     LengthSmall, 
//     Area, 
//     AreaSmall, 
//     Force, 
//     Stress, 
//     Moment,
// }

// impl UnitCategory for StructuralUnitCategory {
//     fn kind(&self) -> UnitKind {
//         match self {
//             Self::Length      => UnitKind::length(),
//             Self::LengthSmall => UnitKind::length(),
//             Self::Area        => UnitKind::area(),
//             Self::AreaSmall   => UnitKind::area(),
//             Self::Force       => UnitKind::force(),
//             Self::Stress      => UnitKind::stress(),
//             Self::Moment      => UnitKind::torque(),
//         }
//     }
// }
 

// #[derive(Debug, Clone, Copy, Default)]
// pub struct StructuralUnitSetting {
//     pub length: LengthUnit,
//     pub length_small: LengthUnit,
//     pub area: LengthUnit,       // Still uses LengthUnit because Area = L^2
//     pub area_small: LengthUnit, // Still uses LengthUnit because Area = L^2
//     pub force: ForceUnit,
//     pub stress: ForceUnit,      // Typically N/mm^2 or N/m^2
//     pub moment: ForceUnit,      // Typically kN*m
// }


// #[derive(Debug, Clone, Copy, PartialEq)]
// pub struct StructuralUnitSetting {
//     pub length: LengthUnit,
//     pub length_small: LengthUnit,
//     pub area: LengthUnit,       // Stored as a LengthUnit (e.g. Meter), used as m²
//     pub area_small: LengthUnit, // Stored as a LengthUnit (e.g. Millimeter), used as mm²
//     pub force: ForceUnit,
//     // For now, let's assume Stress and Moment are derived from Force and Length
//     pub stress_force: ForceUnit,
//     pub stress_length: LengthUnit,
// }


// impl StructuralUnitCategory {
//     /// Maps each application category to its physical UnitKind
//     pub fn physical_kind(&self) -> UnitKind {
//         match self {
//             // Simple Mappings
//             Self::Length | Self::LengthSmall => UnitKind::length(), // L^1
//             Self::Area   | Self::AreaSmall   => UnitKind::area(),   // L^2
            
//             // Compound Mappings
//             Self::Force  => UnitKind::force(),  // M^1 * L^1 * T^-2
//             Self::Stress => UnitKind::stress(), // M^1 * L^-1 * T^-2
//             Self::Moment => UnitKind::torque(), // M^1 * L^2 * T^-2
//         }
//     }
// }


// impl Default for StructuralUnitSetting {
//     fn default() -> Self {
//         Self {
//             length: LengthUnit::Meter,
//             length_small: LengthUnit::Millimeter,
//             area: LengthUnit::Meter,
//             area_small: LengthUnit::Millimeter,
//             force: ForceUnit::Kilonewton,
//             stress_force: ForceUnit::Newton,
//             stress_length: LengthUnit::Millimeter, // results in N/mm² (MPa)
//         }
//     }
// }

// impl UnitSetting<StructuralUnitCategory> for StructuralUnitSetting {
//     fn get_symbol(&self, category: StructuralUnitCategory) -> &'static str {
//         // Dispatch based on category to the correct field in self
//         match category {
//             StructuralUnitCategory::Length => self.length.symbol(),
//             StructuralUnitCategory::LengthSmall => self.length_small.symbol(),
//             StructuralUnitCategory::Area => self.area.symbol(),
//             StructuralUnitCategory::AreaSmall => self.area_small.symbol(),
//             StructuralUnitCategory::Force => self.force.symbol(),
//             // Compound units like Stress/Moment might combine multiple symbols
//             _ => "???" 
//         }
//     }

//     fn to_si(&self, category: StructuralUnitCategory, value: f64) -> f64 {
//         let kind = category.kind();
//         match category {
//             StructuralUnitCategory::Length => self.length.to_si(value, 1),
//             StructuralUnitCategory::LengthSmall => self.length_small.to_si(value, 1),
            
//             // This is where your UnitKind power logic shines!
//             StructuralUnitCategory::Area => self.area.to_si(value, 2),
//             StructuralUnitCategory::AreaSmall => self.area_small.to_si(value, 2),
            
//             StructuralUnitCategory::Force => self.force.to_si(value, 1),
//             // Stress: Force / Length^2
//             StructuralUnitCategory::Stress => {
//                 let f = self.stress_force.to_si(1.0, 1);
//                 let l = self.stress_length.to_si(1.0, 1);
//                 value * (f / (l * l))
//             }
//             _ => value,
//         }
//     }

//     fn from_si(&self, category: StructuralUnitCategory, si_value: f64) -> f64 {
//         match category {
//             StructuralUnitCategory::Length => self.length.from_si(si_value, 1),
//             StructuralUnitCategory::LengthSmall => self.length_small.from_si(si_value, 1),
//             StructuralUnitCategory::Area => self.area.from_si(si_value, 2),
//             _ => si_value,
//         }
//     }
// }
