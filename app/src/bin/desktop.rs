
use std::sync::Arc;
use app::{app::App, renderer::config::RendererConfig };
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, EventLoop},
    window::{Window, WindowId},
};
 struct DesktopApp {
    core: Option<App>,       // Option here only because winit forces Default
}

impl Default for DesktopApp {
    fn default() -> Self {
        Self { core: None }
    }
}

impl ApplicationHandler for DesktopApp {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        tracing::info!("Window resumed — creating window and engine");

        let window = Arc::new(
            event_loop.create_window(Window::default_attributes().with_title("Model Lab")).unwrap()
        );

        let size = window.inner_size();
        tracing::debug!("Window size: {}x{}", size.width, size.height);

        let config = RendererConfig {
            clear_color: wgpu::Color { r: 0.02, g: 0.02, b: 0.02, a: 1.0 },
            ..RendererConfig::default()
        };

        match pollster::block_on(App::new(window, size.width, size.height, config)) {
            Ok(core) => {
                tracing::info!("App init succeeded");
                self.core = Some(core);
            }
            Err(e) => {
                tracing::error!("App init failed: {}", e);
                event_loop.exit();
            }
        }
    }

    fn suspended(&mut self, _event_loop: &ActiveEventLoop) {
        tracing::info!("Window suspended");
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _: WindowId, event: WindowEvent) {
        match &event {
            WindowEvent::CloseRequested  => tracing::info!("Close requested"),
            WindowEvent::Resized(size)   => tracing::debug!("Resized to {}x{}", size.width, size.height),
            WindowEvent::Focused(true)   => tracing::debug!("Window focused"),
            WindowEvent::Focused(false)  => tracing::debug!("Window unfocused"),
            _                            => {}
        }
        if let Some(core) = &mut self.core {
            core.handle_event(&event, event_loop);
        }
    }
}

fn main() {
    tracing_subscriber::fmt::init();
    tracing::info!("Starting Model Lab desktop");

    let event_loop = EventLoop::new().unwrap();
    let mut app    = DesktopApp::default();

    if let Err(e) = event_loop.run_app(&mut app) {
        tracing::error!("Event loop error: {}", e);
    }

    tracing::info!("Model Lab desktop shutdown");
}
 
 