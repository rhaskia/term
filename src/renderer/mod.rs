mod palette;
pub mod terminal;
mod utils;

use crate::terminal::screen::Cell;

use glyph_brush::{
    ab_glyph::{Font, FontRef}, BuiltInLineBreaker, Layout, OwnedSection, OwnedText, Section, VerticalAlign,
};
use termwiz::color::ColorSpec;
use wgpu::{Device, Queue, Surface, SurfaceConfiguration};
use wgpu_text::{BrushBuilder, TextBrush};
use winit::{dpi::PhysicalSize, window::Window};

use std::sync::Arc;

use utils::WgpuUtils;
use palette::Palette;


// pub struct TextRenderer<'a> {
//     brush: TextBrush<FontRef<'a>>,
//     surface: Surface<'a>,
//     queue: Queue,
//     device: Device,
//     section: OwnedSection,
//     config: SurfaceConfiguration,
//     font_size: f32,
//     pub color: [f32; 4],
// }
//
// impl TextRenderer<'_> {
//     pub fn new(window: Arc<Window>) -> TextRenderer<'static> {
//         let (device, queue, surface, config) = WgpuUtils::init(window);
//
//         let font: &[u8] = include_bytes!("../../fonts/JetBrainsMono-Regular.ttf");
//         let brush = BrushBuilder::using_font_bytes(font).unwrap().build(
//             &device,
//             config.width,
//             config.height,
//             config.format,
//         );
//
//         let font_size = 14.;
//         let section = Section::default()
//             .with_bounds((config.width as f32, config.height as f32))
//             .with_layout(
//                 Layout::default()
//                     .v_align(VerticalAlign::Top)
//                     .line_breaker(BuiltInLineBreaker::AnyCharLineBreaker),
//             )
//             .with_screen_position((0.0, 0.0))
//             .to_owned();
//
//         TextRenderer {
//             font_size,
//             device,
//             queue,
//             surface,
//             config,
//             brush,
//             section,
//             color: [1.0, 1.0, 1.0, 1.0],
//         }
//     }
//
//     pub fn render_from_cells(&mut self, cells: Vec<Cell>) {
//         self.section.text = Vec::new();
//
//         for cell in cells {
//             self.section.text.push(
//                 OwnedText::new(cell.char)
//                     .with_scale(self.font_size)
//                     .with_color(cell.attr.fg.to_vec()),
//             );
//         }
//     }
//
//     /// Resizes the screen renderer, text box, and text renderer
//     pub fn resize_view(&mut self, new_size: PhysicalSize<u32>) {
//         self.config.width = new_size.width.max(1);
//         self.config.height = new_size.height.max(1);
//         self.surface.configure(&self.device, &self.config);
//
//         self.section.bounds = (self.config.width as f32, self.config.height as _);
//
//         self.brush.resize_view(
//             self.config.width as f32,
//             self.config.height as f32,
//             &self.queue,
//         );
//     }
//
//     pub fn glyph_size(&mut self) -> (f32, f32) {
//         let font = &self.brush.fonts()[0];
//         let glyph_rect = font.glyph_bounds(&font.glyph_id('M').with_scale(14.));
//
//         (glyph_rect.width(), glyph_rect.height())
//         // match glyph {
//         //     Some(g) => g.glyph.scale,
//         //     None => panic!("No glyphs"),
//         // }
//     }
//
//     pub fn render(&mut self) {
//         match self
//             .brush
//             .queue(&self.device, &self.queue, vec![&self.section])
//         {
//             Ok(_) => (),
//             Err(err) => {
//                 panic!("{err}");
//             }
//         };
//
//         let frame = match self.surface.get_current_texture() {
//             Ok(frame) => frame,
//             Err(_) => {
//                 self.surface.configure(&self.device, &self.config);
//                 self.surface
//                     .get_current_texture()
//                     .expect("Failed to acquire next surface texture!")
//             }
//         };
//         let view = frame
//             .texture
//             .create_view(&wgpu::TextureViewDescriptor::default());
//
//         let mut encoder = self
//             .device
//             .create_command_encoder(&wgpu::CommandEncoderDescriptor {
//                 label: Some("Command Encoder"),
//             });
//
//         {
//             let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
//                 label: Some("Render Pass"),
//                 color_attachments: &[Some(wgpu::RenderPassColorAttachment {
//                     view: &view,
//                     resolve_target: None,
//                     ops: wgpu::Operations {
//                         load: wgpu::LoadOp::Clear(wgpu::Color {
//                             r: 0.0,
//                             g: 0.0,
//                             b: 0.0,
//                             a: 1.,
//                         }),
//                         store: wgpu::StoreOp::Store,
//                     },
//                 })],
//                 depth_stencil_attachment: None,
//                 timestamp_writes: None,
//                 occlusion_query_set: None,
//             });
//
//             self.brush.draw(&mut rpass);
//         }
//
//         self.queue.submit([encoder.finish()]);
//         frame.present();
//     }
// }
//
