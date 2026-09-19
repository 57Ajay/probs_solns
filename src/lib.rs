pub mod cses;
pub mod dsap;

#[allow(unused)]
#[cfg(test)]
mod tests {
    use crate::cses::sorting_and_searching::josephus_problem_II;

    #[test]
    fn test() {
        josephus_problem_II::main();
    }
}
