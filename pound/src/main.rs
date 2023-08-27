use crossterm::event::{self, Event, KeyCode, KeyEvent};
use crossterm::terminal;

use std::time::Duration;

struct CleanUp;

impl Drop for CleanUp {
    fn drop(&mut self) {
        terminal::disable_raw_mode().expect("Unable to disable raw mode")
    }
}

struct Reader;

impl Reader {
    fn read_key(&self) -> std::io::Result<KeyEvent> {
        loop {
            if event::poll(Duration::from_millis(500))? {
                if let Event::Key(event) = event::read()? {
                    println!("{:?}\r", event);
                    return Ok(event);
                }
            } else {
                println!("No input yet\r");
            }
        }
    }
}

struct Editor {
    reader: Reader,
}

impl Editor {
    fn new() -> Self {
        Self { reader: Reader }
    }

    fn process_keypress(&self) -> std::io::Result<bool> {
        match self.reader.read_key()? {
            KeyEvent {
                code: KeyCode::Char('c'),
                modifiers: event::KeyModifiers::CONTROL,
                kind: _,
                state: _,
            } => return Ok(false),
            _ => {}
        }
        Ok(true)
    }

    fn run(&self) -> std::io::Result<bool> {
        self.process_keypress()
    }
}

fn main() -> std::io::Result<()> {
    let _clean_up = CleanUp;
    terminal::enable_raw_mode()?;
    /* modify */
    let editor = Editor::new();
    while editor.run()? {}
    /* end */
    Ok(())
}
