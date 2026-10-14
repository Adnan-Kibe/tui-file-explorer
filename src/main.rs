use std::io;

use crate::{filesystem::App, tui::TUI};

mod tui;
mod filesystem;

fn main() -> io::Result<()> {
    let mut app = App::new()?;

    ratatui::run(|terminal| TUI.app(terminal, &mut app))
}
