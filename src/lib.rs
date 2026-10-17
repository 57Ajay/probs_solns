pub mod cses;
pub mod dsap;
pub mod neetcode;

#[allow(unused)]
#[cfg(test)]
mod tests {
    use crate::neetcode;

    #[test]
    fn test() {
        neetcode::range_sum_query_sum_2d_immutable::main();
    }
}
