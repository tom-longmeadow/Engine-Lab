
 
pub trait Language: Clone + Copy + Default {
    fn id(&self) -> &'static str; // e.g., "en-CA", "es-ES"
}

pub trait DisplayText: 'static + Clone + Copy {
    fn translate<L: Language>(&self, lang: L) -> String;
    fn default_text(&self) -> &'static str;
}
 