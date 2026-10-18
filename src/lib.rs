pub mod cses;
pub mod dsap;
pub mod neetcode;

#[allow(unused)]
#[cfg(test)]
mod tests {
    use crate::neetcode;

    #[test]
    fn test() {
        neetcode::best_time_to_buy_and_sell_stock_II::main();
    }
}
