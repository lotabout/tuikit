use tuikit::attr::*;
use tuikit::term::Term;
use tuikit::key::Key;
use tuikit::event::Event;
use std::cmp::{min, max};

fn main() {
    let term = Term::new().unwrap();
    let mut row = 1;
    let mut col = 0;

    term.print(0, 0, "press arrow key to move the text, (q) to quit");
    term.present();

    while let Ok(ev) = term.poll_event() {
        term.clear();

        let (width, height) = term.term_size().unwrap();
        match ev {
            Event::Key(Key::ESC) | Event::Key(Key::Char('q')) => break,
            Event::Key(Key::Up) => row = max(row-1, 1),
            Event::Key(Key::Down) => row = min(row+1, height-1),
            Event::Key(Key::Left) => col = max(col-1, 0),
            Event::Key(Key::Right) => col = min(col+1, width-1),
            _ => {}
        }

        let attr = Attr{ fg: Color::RED, ..Attr::default() };
        term.print_with_attr(row, col, "Hello World!", attr);
        term.present();
    }
}
