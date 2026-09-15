pub mod cses;
pub mod dsap;

#[allow(unused)]
#[cfg(test)]
mod tests {
    use crate::cses::sorting_and_searching::playlist;

    #[test]
    fn test() {
        playlist::main();
    }
}
