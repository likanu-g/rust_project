use std::io;
use std::io::Read;

use crossterm::terminal;
fn main() {
    terminal::enable_raw_mode().expect("could not turn on Raw mode");
    let mut buf = [0; 1];
    //按q退出，b'q'表示q是一个字节
    while io::stdin().read(&mut buf).expect("Failed to read line") == 1 && buf != [b'q'] {}
}
