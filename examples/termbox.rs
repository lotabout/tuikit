use std::thread;
use std::time::Instant;
use tuikit::event::Event;
use tuikit::key::Key;
use tuikit::term::{Term, TermHeight};

fn main() {
    let term = Term::with_height(TermHeight::Fixed(10));
    let now = Instant::now();
    let th = thread::spawn(move || {
        let (_, height) = term.term_size().unwrap();
        for row in 0..height {
            let _ = term.print(row, 0, format!("{} ", row).as_str());
        }
        let col = 3;

        let _ = term.print(0, col, "Press q to quit");
        let _ = term.present();
        while let Ok(ev) = term.poll_event() {
            if let Event::Key(Key::Char('q')) = ev {
                break;
            }

            let elapsed = now.elapsed();
            let _ = term.print(1, col, format!("{:?}", ev).as_str());
            let _ = term.print(
                height - 1,
                col,
                format!(
                    "time elapsed since program start: {}s + {}ms",
                    elapsed.as_secs(),
                    elapsed.subsec_millis()
                )
                .as_str(),
            );
            let _ = term.present();
        }
    });
    let _ = th.join();
}
