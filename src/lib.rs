pub mod cses;
pub mod dsap;

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test() {
        dsap::llst::llist();
    }
}
