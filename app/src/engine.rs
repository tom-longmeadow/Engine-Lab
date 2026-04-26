use std::sync::Arc;

use winit::{event::WindowEvent, window::Window};
use winit::event_loop::ActiveEventLoop;

pub mod scene;
pub mod input;

use crate::renderer::config::RendererConfig;
use crate::{
    engine::{
        input::InputState, 
        scene::Scene, 
    }, 
    renderer::{
        Renderer, 
        error::RendererError
    }
};
pub struct Engine {
    renderer: Renderer,
    scene:    Box<dyn Scene>,
    input:    InputState,
}

impl Engine {
    
    pub async fn new(
        window: Arc<Window>,
        width:  u32,
        height: u32,
        config: RendererConfig,             
    ) -> Result<Self, RendererError> {
        tracing::info!("Engine initializing {}x{}", width, height);
        let renderer = Renderer::new(window, width, height, config).await?;
        tracing::info!("Engine ready");
        Ok(Self {
            renderer,
            scene: Box::new(scene::EmptyScene),
            input: InputState::new(),
        })
    }

    pub fn handle_event(&mut self, event: &WindowEvent, event_loop: &ActiveEventLoop) {
        self.input.handle(event);
        match event {
            WindowEvent::CloseRequested  => {
                tracing::info!("Close requested");
                event_loop.exit();
            }
            WindowEvent::Resized(size)   => {
                tracing::debug!("Resize {}x{}", size.width, size.height);
                self.renderer.resize(size.width, size.height);
            }
            WindowEvent::RedrawRequested => self.render(event_loop),
            _                            => {}
        }
    }

    fn render(&mut self, event_loop: &ActiveEventLoop) {
        self.input.begin_frame();
        self.scene.update(&self.input);
        self.scene.build_passes(&mut self.renderer);
        match self.renderer.render() {
            Ok(())                          => {}
            Err(RendererError::Outdated) |
            Err(RendererError::Lost)        => {
                tracing::warn!("Surface lost/outdated — reconfiguring");
                self.renderer.reconfigure();
            }
            Err(RendererError::Occluded) |
            Err(RendererError::Timeout)     => {
                tracing::trace!("Frame skipped: occluded or timeout");
            }
            Err(e)                          => {
                tracing::error!("Render failed: {}", e);
                event_loop.exit();
            }
        }
    }
}
 