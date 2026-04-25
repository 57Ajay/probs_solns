pub mod cses;

#[cfg(test)]
mod tests {

    use crate::*;

    #[test]
    fn test() {
        let result_ = cses::introductory_problems::string_reorder::main();
        assert!(result_ != ());
    }
}
