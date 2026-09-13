pub mod cses;
pub mod dsap;

#[allow(unused)]
#[cfg(test)]
mod tests {
    use crate::cses::sorting_and_searching::collecting_numbers;

    #[test]
    fn test() {
        collecting_numbers::main();
    }
}
