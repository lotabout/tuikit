use crate::key::Key;

pub enum Event {
    Key(Key),
    Resize{width: usize, height: usize},

    #[doc(hidden)]
    __Nonexhaustive,
}