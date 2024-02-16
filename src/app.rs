use dioxus::prelude::*;
use std::sync::Arc;
use winit::{event::KeyEvent, window::Window};

use crate::{input::InputManager, renderer::terminal::TerminalApp, terminal::Terminal};

use dioxus_elements::*;

pub fn app(cx: Scope) -> Element {
    cx.render(rsx! {
        style { include_str!("style.css") }
        h1 { "whu" }
        TerminalApp {}
    })
}

// pub struct App<'a> {
//     renderer: TextRenderer<'a>,
//     input: InputManager,
//     terminal: Terminal,
//
//     pub title: String,
// }
//
// impl App<'_> {
//     pub fn setup(window: Arc<Window>) -> App<'static> {
//         App {
//             title: String::from("Term"),
//             renderer: TextRenderer::new(window),
//             input: InputManager::new(),
//             terminal: Terminal::setup().unwrap(),
//         }
//     }
//
//     /// Resizes the rendere and terminal
//     pub fn resize_view(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
//         self.renderer.resize_view(new_size);
//         let glyph_size = self.renderer.glyph_size();
//         self.terminal.resize(new_size, glyph_size);
//     }
//
//     pub fn render(&mut self) {
//         self.renderer.render();
//     }
//
//     /// Mostly a handler of Actions that the terminal gives out
//     pub fn update(&mut self) {
//         loop {
//             let action = match self.terminal.pty.rx.try_recv() {
//                 Ok(a) => a,
//                 _ => break,
//             };
//
//             println!("cursor {}, {}", self.terminal.cursor.x, self.terminal.cursor.y);
//
//             
//         }
//
//         // TODO: only render when needed
//         // im sure dixous will fix this issue
//         self.renderer.render_from_cells(self.terminal.get_cells());
//     }
//
//     pub fn handle_edit(&mut self, edit: Edit) {
//         use EraseInLine::*;
//         match edit {
//             //Edit::EraseInLine(EraseToEndOfLine) => {}
//             Edit::EraseInLine(e) => self.terminal.erase_in_line(e),
//             _ => println!("Edit {:?}", edit),
//         }
//     }
//
//
//
//     /// Handles what happends with keyboard inputs
//     pub fn handle_input(&mut self, key: KeyEvent) {
//         use crate::input::Input;
//
//         match self.input.handle_input(key) {
//             Input::String(s) => self.terminal.pty.writer.write_all(s.as_bytes()),
//             Input::Control(c) => match c.as_str() {
//                 "c" => self.terminal.pty.writer.write_all("\x03".as_bytes()),
//                 _ => Ok(()),
//             },
//             Input::None => Ok(()),
//         }
//         .unwrap();
//     }
// }
