pub struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        1
    }
}

#[cfg(test)]
mod tests {
    use crate::prob_0011_container_with_most_water::{self, Solution};

    #[test]
    fn solution_works() {
        assert_eq!(Solution::max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]), 49);
        assert_eq!(Solution::max_area(vec![1, 1]), 1);
    }
}
