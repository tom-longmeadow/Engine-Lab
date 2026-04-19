

// /// An example of making the structural data
// /// NOTE: any secondary key, like Member joint_i_id: Id64,
// /// the id type must be the same as the ComponentId you are using 
// /// in StructuralKind or you will get errors using the component.
// #[derive(Debug, Clone, PartialEq)]
// pub enum StructuralData {
//     Joint {
//         x: f64,
//         y: f64,
//         z: f64,
//     },
//     Member {
//         joint_i_id: Id64, // joint ID. 
//         joint_j_id: Id64,
//         section_id: Id64,
//     },
//     Material {
//         e: f64, // Modulus of Elasticity
//     },
//     Section {
//         area: f64,
//         ix: f64, // Moment of Inertia
//         material_id: Id64,
//     },

//     Panel {
//         joint_ids: Vec<Id64>,
//         thickness: f64,
//     },
// }

// impl ComponentData for StructuralData {
//     type Kind = StructuralKind;

//     fn kind(&self) -> Self::Kind {
//         match self {
//             Self::Joint { .. } => StructuralKind::Joint,
//             Self::Member { .. } => StructuralKind::Member,
//             Self::Section { .. } => StructuralKind::Section,
//             Self::Panel { .. } => StructuralKind::Panel,
//             Self::Material { .. } => StructuralKind::Material,
//         }
//     }
// }