use std::io;
use tuikit::attr::{Attr, Color};
use tuikit::output::Output;
use tuikit::screen::Screen;

fn main() {
    let mut output = Output::new(Box::new(io::stdout())).unwrap();
    let (width, height) = output.terminal_size().unwrap();
    let mut screen = Screen::new(width, height);

    for fg in 0..=255 {
        screen.print(
            fg / 16,
            (fg % 16) * 5,
            format!("{:5}", fg).as_str(),
            Attr {
                fg: Color::AnsiValue(fg as u8),
                ..Attr::default()
            },
        );
    }

    screen.set_cursor(15, 80);
    let commands = screen.present();

    commands.into_iter().for_each(|cmd| output.execute(cmd));
    output.flush();

    screen.print(0, 78, "HELLO WORLD", Attr::default());
    let commands = screen.present();

    commands.into_iter().for_each(|cmd| output.execute(cmd));
    output.flush();

    for bg in 0..=255 {
        screen.print(
            bg / 16,
            (bg % 16) * 5,
            format!("{:5}", bg).as_str(),
            Attr {
                bg: Color::AnsiValue(bg as u8),
                ..Attr::default()
            },
        );
    }
    let commands = screen.present();
    commands.into_iter().for_each(|cmd| output.execute(cmd));
    output.flush()
}
