use tuikit::prelude::*;

struct Model(String);

impl Draw for Model {
    fn draw(&self, canvas: &mut dyn Canvas) -> Result<()> {
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
        match ev {
            Event::Key(Key::Char('q')) | Event::Key(Key::Ctrl('c')) => break,
            _ => (),
        }
        let _ = term.print(0, 0, "press 'q' to exit");

        let inner_win = Win::new(&model).border(true);

        let win = Win::new(&inner_win)
            .margin(Size::Percent(10))
            .padding(1)
            .border(true)
            .border_top_attr(Color::BLUE)
            .border_right_attr(Color::YELLOW)
            .border_bottom_attr(Color::RED)
            .border_left_attr(Color::GREEN);

        let _ = term.draw(&win);
        let _ = term.present();
    }
}
