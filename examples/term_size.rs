use rustui::output::Output;
use std::io;

fn main() {
    let output = Output::new(io::stdout(), None);
    let (width, height) = output.terminal_size().unwrap();
    println!("width: {}, height: {}", width, height);
}

