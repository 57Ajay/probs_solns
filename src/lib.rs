pub mod cses;
pub mod dsap;

#[allow(unused)]
#[cfg(test)]
mod tests {
    use crate::cses::dynamic_programming::{dice_combinations, minimizing_coins};
    use crate::cses::sorting_and_searching::nested_ranges_check;

    #[test]
    fn test() {
        // nested_ranges_check::main();
        minimizing_coins::main();
    }
}
