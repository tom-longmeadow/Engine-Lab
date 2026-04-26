use crate::{
    engine::input::InputState, 
    renderer::{Renderer, pass::text::TextRenderPass}
};



pub trait Scene {
    fn update(&mut self, input: &InputState);
    fn build_passes(&self, renderer: &mut Renderer);
}

 