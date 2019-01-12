use crate::key::Key;

#[derive(Debug)]
pub enum Event {
    Key(Key),
    Resize {
        width: usize,
        height: usize,
    },

    #[doc(hidden)]
    __Nonexhaustive,
}
