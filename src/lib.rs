pub mod attr;
pub mod input;
pub mod keys;
pub mod output;
pub mod raw;
mod sys;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
