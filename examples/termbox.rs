use std::borrow::BorrowMut;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};
use tuikit::attr::Attr;
use tuikit::event::Event;
use tuikit::key::Key;
use tuikit::term::{Term, TermHeight};

fn main() {
    let mut term = Term::with_height(TermHeight::Fixed(10));
    let now = Instant::now();
    let th = thread::spawn(move || {
        let (width, height) = term.term_size().unwrap();
        for row in 0..height {
            term.print(row, 0, format!("{} ", row).as_str());
        }
        let col = 3;

        term.print(0, col, "Press q to quit");
        term.present();
        while let Ok(ev) = term.poll_event() {
            if let Event::Key(Key::Char('q')) = ev {
                break;
            }

            let elapsed = now.elapsed();
            term.print(1, col, format!("{:?}", ev).as_str());
            term.print(height-1, col, format!("time elapsed since program start: {}s + {}ms", elapsed.as_secs(), elapsed.subsec_millis()).as_str());
            term.present();
        }
    });
    th.join();
}
