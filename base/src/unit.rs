pub mod base_unit; 
pub mod kind;
pub mod category;
pub mod settings;

pub use base_unit::*; 
pub use kind::*;
pub use category::*;
pub use settings::*;

use crate::model::ModelConfig;

pub type Simple = SimpleUnit;
pub type Compound = CompoundUnit;
pub type Temperature = TemperatureUnit;
 


pub struct UnitSystem<Config: ModelConfig> {
    pub file: Config::UnitSetting,
    pub display: Config::UnitSetting,
}

impl<Config: ModelConfig> UnitSystem<Config> {
    pub fn new(file: Config::UnitSetting, display: Config::UnitSetting) -> Self {
        Self { file, display }
    }

    /// Returns the symbol of a specific category for UI labels.
    pub fn display_symbol(&self, category: Config::UnitCategory) -> String {
        self.display.get_kind(category).symbol()
    }

    /// Converts a value from file units to display units for a specific category.
    pub fn file_to_display(&self, value: f64, category: Config::UnitCategory) -> f64 {
        let file_kind = self.file.get_kind(category);
        let display_kind = self.display.get_kind(category);

        let base = file_kind.to_base(value);
        display_kind.from_base(base)
    }

    /// Converts a value from display units back to file units for a specific category.
    pub fn display_to_file(&self, value: f64, category: Config::UnitCategory) -> f64 {
        let file_kind = self.file.get_kind(category);
        let display_kind = self.display.get_kind(category);

        let base = display_kind.to_base(value);
        file_kind.from_base(base)
    }
}