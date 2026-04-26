use crate::{
    engine::input::InputState, 
    renderer::Renderer
};



pub trait Scene {
    fn update(&mut self, input: &InputState);
    fn build_passes(&self, renderer: &mut Renderer);
}
pub struct EmptyScene;
impl Scene for EmptyScene {
    fn update(&mut self, _input: &InputState) {}
    fn build_passes(&self, _renderer: &mut Renderer) {}
}