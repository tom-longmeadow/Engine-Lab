


// /// 2. Create a type alias for easy use elsewhere
// pub type StructuralModel = Model<StructuralConfig>;

// fn main() {
//     // 3. Initialize the Registry and Settings
//     let registry = HashMapRegistry::new();
//     let settings = UnitSettings::new(MyUnitSetting, MyUnitSetting);
    
//     // 4. Instantiate the Model
//     let mut model = StructuralModel::new(registry, settings);

//     // Example: Inserting a Joint
//     let joint_id = Id64(1);
//     let joint_data = StructuralData::Joint { x: 1.0, y: 2.0, z: 0.0 };
    
//     model.insert(joint_id, joint_data).expect("Failed to insert joint");

//     // Example: Using the Component wrapper
//     let member = Component {
//         id: Id64(10),
//         data: StructuralData::Member {
//             joint_i_id: Id64(1),
//             joint_j_id: Id64(2),
//             section_id: Id64(100),
//         },
//     };
    
//     model.insert_comp(member).expect("Failed to insert member");
// }