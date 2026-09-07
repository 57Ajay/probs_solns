pub mod cses;
pub mod dsap;

#[allow(unused)]
#[cfg(test)]
mod tests {
    use std::assert_eq;

    use crate::cses::introductory_problems::grid_path_description::solve;

    #[test]
    fn test_sample() {
        let s = "??????R??????U??????????????????????????LD????D?";
        assert_eq!(solve(s), 201);
    }

    #[test]
    fn test_all_question_marks() {
        let s = "?".repeat(48);
        assert_eq!(solve(&s), 88418);
    }
}
