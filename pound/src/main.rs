use std::io;
use std::io::Read;

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
    let mut buf = [0; 1];
    //按q退出，b'q'表示q是一个字节
    while io::stdin().read(&mut buf).expect("Failed to read line") == 1 && buf != [b'q'] {
        let character = buf[0] as char;
        //判断是否是控制字符，因为控制字符在ASCII码中无法打印，
        //ASCII码0-31和127为为控制字符
        //ASCII码32-126为可打印字符
        if character.is_control() {
            println!("{}\r", character as u8)
        } else {
            println!("{}\r", character)
        }
    }
}
