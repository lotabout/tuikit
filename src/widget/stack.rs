use crate::canvas::{Canvas, Result};
use crate::draw::Draw;
use crate::event::Event;
use crate::widget::{Rectangle, Widget};

/// A stack of widgets, will draw the including widgets back to front
pub struct Stack<'a, Message = ()> {
    inner: Vec<Box<dyn Widget<Message> + 'a>>,
}

impl<'a, Message> Stack<'a, Message> {
    pub fn new() -> Self {
        Self { inner: vec![] }
    }

    pub fn top(mut self, widget: impl Widget<Message> + 'a) -> Self {
        self.inner.push(Box::new(widget));
        self
    }

    pub fn bottom(mut self, widget: impl Widget<Message> + 'a) -> Self {
        self.inner.insert(0, Box::new(widget));
        self
    }
}

impl<'a, Message> Draw for Stack<'a, Message> {
    fn draw(&self, canvas: &mut dyn Canvas) -> Result<()> {
        for widget in self.inner.iter() {
            widget.draw(canvas)?
        }

        Ok(())
    }
}

impl<'a, Message> Widget<Message> for Stack<'a, Message> {
    fn size_hint(&self) -> (Option<usize>, Option<usize>) {
        // max of the inner widgets
        let width = self
            .inner
            .iter()
            .map(|widget| widget.size_hint().0)
            .max()
            .unwrap_or(None);
        let height = self
            .inner
            .iter()
            .map(|widget| widget.size_hint().1)
            .max()
            .unwrap_or(None);
        (width, height)
    }

    fn on_event(&self, event: Event, rect: Rectangle) -> Vec<Message> {
        // like javascript's capture, from top to bottom
        for widget in self.inner.iter().rev() {
            let message = widget.on_event(event, rect);
            if !message.is_empty() {
                return message;
            }
        }
        vec![]
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod test {
    use super::*;

    struct WinHint {
        pub width_hint: Option<usize>,
        pub height_hint: Option<usize>,
    }

    impl Draw for WinHint {
        fn draw(&self, _canvas: &mut dyn Canvas) -> Result<()> {
            unimplemented!()
        }
    }

    impl Widget for WinHint {
        fn size_hint(&self) -> (Option<usize>, Option<usize>) {
            (self.width_hint, self.height_hint)
        }
    }

    #[test]
    fn size_hint() {
        let stack = Stack::new().top(WinHint {
            width_hint: None,
            height_hint: None,
        });
        assert_eq!((None, None), stack.size_hint());

        let stack = Stack::new().top(WinHint {
            width_hint: Some(1),
            height_hint: Some(1),
        });
        assert_eq!((Some(1), Some(1)), stack.size_hint());

        let stack = Stack::new()
            .top(WinHint {
                width_hint: Some(1),
                height_hint: Some(2),
            })
            .top(WinHint {
                width_hint: Some(2),
                height_hint: Some(1),
            });
        assert_eq!((Some(2), Some(2)), stack.size_hint());

        let stack = Stack::new()
            .top(WinHint {
                width_hint: None,
                height_hint: None,
            })
            .top(WinHint {
                width_hint: Some(2),
                height_hint: Some(1),
            });
        assert_eq!((Some(2), Some(1)), stack.size_hint());
    }
}
