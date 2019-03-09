use tuikit::attr::{Attr, Color};
use tuikit::canvas::{Canvas, Result};
use tuikit::container::{HSplit, Size, Split, Win};
use tuikit::draw::Draw;
use tuikit::event::{Event, Key};
use tuikit::term::{Term, TermHeight};

struct Model(String);

impl Draw for Model {
    fn draw(&self, canvas: &mut Canvas) -> Result<()> {
        let (width, height) = canvas.size()?;
        let message_width = self.0.len();
        let left = (width - message_width) / 2;
        let top = height / 2;
        let _ = canvas.print(top, left, &self.0);
        Ok(())
    }
}

fn main() {
    let term = Term::with_height(TermHeight::Percent(50)).unwrap();
    let model = Model("Hey, I'm in middle!".to_string());

    while let Ok(ev) = term.poll_event() {
        if let Event::Key(Key::Char('q')) = ev {
            break;
        }
        let _ = term.print(0, 0, "press 'q' to exit");

        let mut canvas = term.get_canvas();
        let inner_win = Win::new(&model).border(true);
        let hsplit = HSplit::default()
            .split(Split::new(&inner_win).basis(Size::Percent(30)))
            .split(Split::new(&inner_win));

        hsplit.draw(&mut canvas);
        let _ = term.present();
    }
}
