use crate::event::Event;

/// terminal
pub struct Terminal {}

impl Terminal {
    /// wait an event up to timeout_mills milliseconds and return it
    fn peek_event(&mut self, timeout_mills: u32) -> Option<Event> {
        None
    }

    /// wait for an event and return it
    fn poll_event(&mut self) -> Option<Event> {
        None
    }
}
