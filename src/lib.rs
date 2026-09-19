pub mod cses;
pub mod dsap;

#[allow(unused)]
#[cfg(test)]
mod tests {
    use crate::cses::sorting_and_searching::distinct_values_subsequences;

    #[test]
    fn test() {
        distinct_values_subsequences::main();
    }
}
