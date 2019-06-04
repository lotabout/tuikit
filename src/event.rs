//! events a `Term` could return

pub use crate::key::Key;

#[derive(Eq, PartialEq, Hash, Debug)]
pub enum Event {
    Key(Key),
    Resize {
        width: usize,
        height: usize,
    },
    Restarted,
    /// user defined signal 1
    User1,
    /// user defined signal 2
    User2,

    #[doc(hidden)]
    __Nonexhaustive,
}
