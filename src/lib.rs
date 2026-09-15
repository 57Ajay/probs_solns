pub mod cses;
pub mod dsap;

#[allow(unused)]
#[cfg(test)]
mod tests {
    use crate::cses::sorting_and_searching::collecting_numbers_II;

    #[test]
    fn test() {
        collecting_numbers_II::main();
    }
}
