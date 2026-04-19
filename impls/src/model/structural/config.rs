

// pub struct StructuralConfig;

// impl ModelConfig for StructuralConfig {
//     type Id = Id64;
//     type Data = StructuralData;
//     type Registry = HashMapRegistry<StructuralData>;
    
//     // Wire in your specific unit and language types
//     type Category = MyUnitCategory; 
//     type Setting = MyUnitSetting;
//     type Lang = CanadianLanguage;
//     type Display = CommonDisplayText;
// }

// // // Now you can instantiate your model
// // let registry = HashMapRegistry::new();
// // let settings = UnitSettings::new(MyUnitSetting, MyUnitSetting);
// // let mut model = Model::<StructuralModelConfig>::new(registry, settings);
