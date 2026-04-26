use crate::renderer::pass::RenderPass;

pub struct TextRenderPass {
    atlas:    glyphon::TextAtlas,
    renderer: glyphon::TextRenderer,
    viewport: glyphon::Viewport,
    buffers:  Vec<glyphon::Buffer>,  // one per visible cell
}

impl RenderPass for TextRenderPass {
    fn prepare(&mut self, device: &wgpu::Device, config: &wgpu::SurfaceConfiguration) {
        // rebuild viewport for new size
    }

    fn update(&mut self, queue: &wgpu::Queue) {
        // upload new text buffers for visible rows
    }

    fn draw<'a>(&'a mut self, pass: &mut wgpu::RenderPass<'a>) {
        self.renderer.render(&self.atlas, &self.viewport, pass).unwrap();
    }
}