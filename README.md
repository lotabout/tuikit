[![Crates.io](https://img.shields.io/crates/v/tuikit.svg)](https://crates.io/crates/tuikit)
## Tuikit

Tuikit is a TUI library for writing terminal UI applications. Highlights:

- Thread safe.
- Support non-fullscreen mode as well as fullscreen mode.
- Support `Alt` keys, mouse events, etc.
- Buffering for efficient rendering.

Tuikit is modeld after [termbox](https://github.com/nsf/termbox) which views the
terminal as a table of fixed-size cells and input being a stream of structured
messages.

## Usage

In your `Cargo.toml` add the following:

```toml
[dependencies]
tuikit = "*"
```

And if you'd like to use the latest snapshot version:

```toml
[dependencies]
tuikit = { git = "https://github.com/lotabout/tuikit.git" }
```

Here is an example (could also be run by `cargo run --example hello-world`):

```rust
use tuikit::prelude::*;
use std::cmp::{min, max};

fn main() {
    let term = Term::with_height(TermHeight::Percent(30)).unwrap();
    let mut row = 1;
    let mut col = 0;

    let _ = term.print(0, 0, "press arrow key to move the text, (q) to quit");
    let _ = term.present();

    while let Ok(ev) = term.poll_event() {
        let _ = term.clear();
        let _ = term.print(0, 0, "press arrow key to move the text, (q) to quit");

        let (width, height) = term.term_size().unwrap();
        match ev {
            Event::Key(Key::ESC) | Event::Key(Key::Char('q')) => break,
            Event::Key(Key::Up) => row = max(row-1, 1),
            Event::Key(Key::Down) => row = min(row+1, height-1),
            Event::Key(Key::Left) => col = max(col, 1)-1,
            Event::Key(Key::Right) => col = min(col+1, width-1),
            _ => {}
        }

        let attr = Attr{ fg: Color::RED, ..Attr::default() };
        let _ = term.print_with_attr(row, col, "Hello World! 你好！今日は。", attr);
        let _ = term.set_cursor(row, col);
        let _ = term.present();
    }
}
```

## Future Plans

Goal:
- "Layout System". Something like the CSS "flexbox" for managing layouts of
    TUI applications.

Not Goal:
- Windows support due to my lack of windows experience.
- TUI Widges.

## References

`Tuikit` borrows ideas from lots of other projects:

- [rustyline](https://github.com/kkawakam/rustyline) Readline Implementation in Rust.
    - How to enter the raw mode.
    - Part of the keycode parsing logic.
- [termion](https://gitlab.redox-os.org/redox-os/termion) A bindless library for controlling terminals/TTY.
    - How to parse mouse events.
    - How to enter raw mode.
- [rustbox](https://github.com/gchp/rustbox) and [termbox](https://github.com/nsf/termbox)
    - The idea of viewing terminal as table of fixed cells.
- [termfest](https://github.com/agatan/termfest) Easy TUI library written in Rust
    - The buffering idea.
