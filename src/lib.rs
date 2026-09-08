pub mod cses;
pub mod dsap;

#[allow(unused)]
#[cfg(test)]
mod tests {
    use crate::cses::sorting_and_searching::apartments;

    #[test]
    fn test_apartments() {
        apartments::main();
    }

    // use crate::cses::{self, sorting_and_searching::distinct_numbers::count_unique_keys};
    //
    // #[test]
    // fn test() {
    //     let input = "3\n2 3 2 2 3";
    //     let result = count_unique_keys(input.as_bytes());
    //
    //     assert_eq!(result, 2);
    // }
    //
    // #[test]
    // fn test_cses_sample() {
    //     let input = "5\n2 3 2 2 3";
    //     assert_eq!(count_unique_keys(input.as_bytes()), 2);
    // }
    //
    // #[test]
    // fn test_single_element() {
    //     let input = "1\n1000000000";
    //     assert_eq!(count_unique_keys(input.as_bytes()), 1);
    // }
    //
    // #[test]
    // fn test_zero_elements() {
    //     let input = "0\n";
    //     assert_eq!(count_unique_keys(input.as_bytes()), 0);
    // }
    //
    // #[test]
    // fn test_multiline_input() {
    //     let input = "4\n10\n20\n10\n30\n";
    //     assert_eq!(count_unique_keys(input.as_bytes()), 3);
    // }

    // use crate::cses::introductory_problems::grid_path_description::solve;
    //
    // #[test]
    // fn test_sample() {
    //     let s = "??????R??????U??????????????????????????LD????D?";
    //     assert_eq!(solve(s), 201);
    // }
    //
    // #[test]
    // fn test_all_question_marks() {
    //     let s = "?".repeat(48);
    //     assert_eq!(solve(&s), 88418);
    // }
}
