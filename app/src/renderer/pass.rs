pub mod text;

pub trait RenderPass {
    fn prepare(&mut self, device: &wgpu::Device, config: &wgpu::SurfaceConfiguration);
    fn update(&mut self, queue: &wgpu::Queue);
    fn draw<'a>(&'a mut self, pass: &mut wgpu::RenderPass<'a>);  // &mut self
}
