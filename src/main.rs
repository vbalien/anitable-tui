use std::io;

use termion::event::Key;
use termion::input::MouseTerminal;
use termion::raw::IntoRawMode;
use termion::screen::AlternateScreen;
use tui::backend::TermionBackend;
use tui::Terminal;

mod app;
mod util;
use util::event::{Event, Events};
use app::ui;
use app::App;

#[tokio::main]
async fn main() -> Result<(), failure::Error> {
    // Terminal initialization
    let stdout = io::stdout().into_raw_mode()?;
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.hide_cursor()?;

    let events = Events::new();

    // App
    let mut app = App::new();
    app.get_list().await;

    // Main loop
    loop {
        ui::draw(&mut terminal, &app)?;
        match events.next()? {
            Event::Input(input) => match input {
                Key::Char(c) => app.on_key(c).await,
                Key::Right => app.on_next().await,
                Key::Left => app.on_prev().await,
                Key::Up => app.on_up(),
                Key::Down => app.on_down(),
                _ => {}
            },
            _ => {}
        }
        if app.should_quit {
            break;
        }
    }
    Ok(())
}
