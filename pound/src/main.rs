use crossterm::event::{self, Event, KeyCode, KeyEvent};
use crossterm::terminal;

struct CleanUp;

impl Drop for CleanUp {
    fn drop(&mut self) {
        terminal::disable_raw_mode().expect("Could not disable raw mode")
    }
}

fn main() {
    let _clean_up = CleanUp;
    terminal::enable_raw_mode().expect("could not turn on Raw mode");

    loop {
        if let Event::Key(event) = event::read().expect("failed to read line") {
            match event {
                KeyEvent {
                    code: KeyCode::Char('q'),
                    modifiers: event::KeyModifiers::NONE,
                    kind: _,
                    state: _,
                } => break,
                _ => {}
            }
            println!("{:?}\r", event);
        };
    }
}
