

// pub trait RenderPass {
//     /// Called once on init or window resize — builds pipeline, buffers, bind groups
//     fn prepare(&mut self, device: &wgpu::Device, config: &wgpu::SurfaceConfiguration);

//     /// Called each frame before encode — upload data to GPU (uniforms, vertex buffers)
//     fn update(&mut self, queue: &wgpu::Queue);

//     /// Called each frame — records draw calls into the encoder
//     fn draw<'a>(&'a self, pass: &mut wgpu::RenderPass<'a>);
// }

// pub struct Renderer {
//     surface:  wgpu::Surface,
//     device:   wgpu::Device,
//     queue:    wgpu::Queue,
//     config:   wgpu::SurfaceConfiguration,
//     passes:   Vec<Box<dyn RenderPass>>,
// }

// impl Renderer {
//     pub fn add_pass(&mut self, pass: Box<dyn RenderPass>) {
//         self.passes.push(pass);
//     }

//     pub fn resize(&mut self, width: u32, height: u32) {
//         self.config.width  = width;
//         self.config.height = height;
//         self.surface.configure(&self.device, &self.config);
//         for pass in &mut self.passes {
//             pass.prepare(&self.device, &self.config);
//         }
//     }

//     pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
//         let output  = self.surface.get_current_texture()?;
//         let view    = output.texture.create_view(&wgpu::TextureViewDescriptor::default());

//         // update all passes before encoding
//         for pass in &mut self.passes {
//             pass.update(&self.queue);
//         }

//         let mut encoder = self.device.create_command_encoder(
//             &wgpu::CommandEncoderDescriptor { label: Some("render_encoder") }
//         );

//         {
//             let mut wgpu_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
//                 label: Some("main_pass"),
//                 color_attachments: &[Some(wgpu::RenderPassColorAttachment {
//                     view: &view,
//                     resolve_target: None,
//                     ops: wgpu::Operations {
//                         load:  wgpu::LoadOp::Clear(wgpu::Color::BLACK),
//                         store: wgpu::StoreOp::Store,
//                     },
//                 })],
//                 depth_stencil_attachment: None,
//                 ..Default::default()
//             });

//             for pass in &self.passes {
//                 pass.draw(&mut wgpu_pass);
//             }
//         }

//         self.queue.submit(std::iter::once(encoder.finish()));
//         output.present();
//         Ok(())
//     }
// }