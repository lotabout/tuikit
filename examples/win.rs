use tuikit::attr::{Attr, Color};
use tuikit::canvas::{Canvas, Result};
use tuikit::container::{Size, Win};
use tuikit::draw::Draw;
use tuikit::event::{Event, Key};
use tuikit::term::{Term, TermHeight};

struct Model {
    pub top: usize,
    pub left: usize,
    pub message: String,
}

impl Draw for Model {
    fn draw(&self, canvas: &mut Canvas) -> Result<()> {
        let (width, height) = canvas.size()?;
        let message_width = self.message.len();
        let left = (width - message_width) / 2;
        let top = height / 2;
        let _ = canvas.print(top, left, &self.message);
        Ok(())
    }
}

fn main() {
    let term = Term::with_height(TermHeight::Percent(50)).unwrap();
    let model = Model {
        top: 0,
        left: 0,
        message: "Hey, I'm in middle!".to_string(),
    };

    while let Ok(ev) = term.poll_event() {
        if let Event::Key(Key::Char('q')) = ev {
            break;
        }
        let _ = term.print(0, 0, "press 'q' to exit");

        let mut canvas = term.get_canvas();
        let inner_win = Win::new(&model).border(true);

        let win = Win::new(&inner_win)
            .margin(Size::Percent(10))
            .padding(Size::Fixed(1))
            .border(true)
            .border_top_attr(Attr {
                fg: Color::BLUE,
                ..Attr::default()
            })
            .border_right_attr(Attr {
                fg: Color::YELLOW,
                ..Attr::default()
            })
            .border_bottom_attr(Attr {
                fg: Color::RED,
                ..Attr::default()
            })
            .border_left_attr(Attr {
                fg: Color::GREEN,
                ..Attr::default()
            });

        let _ = win.draw(&mut canvas);
        let _ = term.present();
    }
}
