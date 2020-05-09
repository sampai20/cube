extern crate termion;
extern crate serde;
extern crate serde_json;

mod app;
mod event_handler;
mod scramble;
mod statistic;
mod timer;
mod ui;
mod data;

use event_handler::Event;

use termion::raw::IntoRawMode;

use std::io;
use tui::backend::TermionBackend;
use tui::Terminal;

use app::App;

fn main() -> Result<(), io::Error> {
    let data = data::read_from_file();

    let mut app = match data {
        Ok(data) => App::with_data(data),
        Err(_) => App::default(),
    };

    let stdout = io::stdout().into_raw_mode()?;
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    terminal.clear()?;
    terminal.hide_cursor()?;

    loop {
        if let Ok(event) = app.process_event() {
            match event {
                Event::Input(c) => match c {
                    'q' => break,
                    ' ' => app.toggle(),
                    _ => continue,
                },
                Event::InspectionInterrupt => {
                    app.inspection_countdown();
                }
                Event::DrawInterrupt => terminal.draw(|mut f| ui::draw(&mut f, &app)).unwrap(),
            };
        }
    }

    data::write_to_file(app)?;

    terminal.clear()?;
    terminal.show_cursor()?;

    Ok(())
}
