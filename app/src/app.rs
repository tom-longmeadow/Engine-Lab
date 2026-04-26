
use std::sync::Arc; 
use winit::{
    event::WindowEvent,
    event_loop::ActiveEventLoop,
    window::Window,
};

use crate::{engine::Engine, renderer::{config::RendererConfig, error::RendererError}};
pub struct App {
    pub engine: Engine,           
}

impl App {
    pub async fn new(
        window: Arc<Window>,
        width:  u32,
        height: u32,
        config: RendererConfig,              // ← caller provides it
    ) -> Result<Self, RendererError> {
        let engine = Engine::new(window, width, height, config).await?;
        Ok(Self { engine })
    }

    pub fn handle_event(&mut self, event: &WindowEvent, event_loop: &ActiveEventLoop) {
        self.engine.handle_event(event, event_loop);
    }
}