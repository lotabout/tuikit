use tuikit::prelude::*;

struct Fit(String);

impl Draw for Fit {
    fn draw(&self, canvas: &mut Canvas) -> Result<()> {
        let (_width, height) = canvas.size()?;
        let top = height / 2;
        let _ = canvas.print(top, 0, &self.0);
        Ok(())
    }

    fn size_hint(&self) -> (Option<usize>, Option<usize>) {
        (Some(self.0.len()), None)
    }
}

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
    let fit = Fit("Short Text That Fits".to_string());

    while let Ok(ev) = term.poll_event() {
        if let Event::Key(Key::Char('q')) = ev {
            break;
        }
        let _ = term.print(0, 0, "press 'q' to exit");

        let hsplit = HSplit::default()
            .split(
                VSplit::default()
                    .shrink(0)
                    .grow(0)
                    .split(Win::new(&fit).border(true))
                    .split(Win::new(&fit).border(true)),
            )
            .split(Win::new(&model).border(true));

        let _ = term.draw(&hsplit);
        let _ = term.present();
    }
}
