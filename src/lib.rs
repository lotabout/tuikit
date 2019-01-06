pub mod attr;
pub mod color;
pub mod event;
pub mod input;
pub mod key;
pub mod output;
pub mod raw;
pub mod screen;
mod sys;
pub mod terminal;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
