pub mod cses;
pub mod dsap;
pub mod neetcode;

#[allow(unused)]
#[cfg(test)]
mod tests {
    use crate::neetcode;

    #[test]
    fn test() {
        neetcode::four_sum::main();
    }
}
