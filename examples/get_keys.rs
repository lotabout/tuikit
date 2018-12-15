use tuikit::input::KeyBoard;
use tuikit::keys::Key;
use tuikit::raw::IntoRawMode;

fn main() {
    let _stdout = std::io::stdout().into_raw_mode();
    let mut keyboard = KeyBoard::new_with_tty();
    while let Ok(key) = keyboard.next_key() {
        if key == Key::Char('q') {
            break;
        }
        println!("print: {:?}", key);
    }
}
