pub mod attr;
pub mod input;
pub mod key;
pub mod output;
pub mod raw;
pub mod screen;
pub mod color;
pub mod event;
mod sys;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
