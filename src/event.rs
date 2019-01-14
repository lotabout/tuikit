//! events a `Term` could return

pub use crate::key::Key;

#[derive(Debug)]
pub enum Event {
    Key(Key),
    Resize {
        width: usize,
        height: usize,
    },
    Restarted,

    #[doc(hidden)]
    __Nonexhaustive,
}
