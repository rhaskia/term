use dioxus::prelude::*;

use crate::renderer::terminal::TerminalApp;
use crate::terminal::pty::PseudoTerminalSystem;

pub mod header;
pub mod terminal;

#[component]
pub fn TerminalSplit() -> Element {
    // Set up vector arrangement
    let pty_system = use_signal(|| PseudoTerminalSystem::setup());

    rsx! {
        TerminalApp { index: 0, pty_system },
    }
}
