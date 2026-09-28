pub mod cses;
pub mod dsap;

#[allow(unused)]
#[cfg(test)]
mod tests {
    use crate::cses::dynamic_programming::{
        coin_combinations_I, coin_combinations_II, dice_combinations, grid_paths_I,
        minimizing_coins, removing_digits,
    };
    use crate::cses::sorting_and_searching::nested_ranges_check;

    #[test]
    fn test() {
        // nested_ranges_check::main();
        grid_paths_I::main();
    }
}
