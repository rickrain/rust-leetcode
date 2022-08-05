pub struct Solution;

// Runtime: 0 ms, faster than 100.00% of Rust online submissions for Two Sum.
// Memory Usage: 2.4 MB, less than 33.11% of Rust online submissions for Two Sum.

use std::collections::HashMap;

#[allow(dead_code)]
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map: HashMap<i32, i32> = HashMap::with_capacity(nums.len() * 2);

        for (index, i) in nums.iter().enumerate() {
            let diff = target - i;
            match map.get(&diff) {
                Some(x) => return vec![*x, index.try_into().unwrap()],
                None => map.insert(*i, index.try_into().unwrap()),
            };
        }

        vec![]
    }
}

// nums: 2 6 9 1 5 3
// target: 4
// map:
//  2 : 0
//  6 : 1
//  9 : 2
//  1 : 3
//  5 : 4
//  3 : 5

#[cfg(test)]
mod tests {
    use crate::prob_0001_two_sum::Solution;

    #[test]
    fn solution_works() {
        assert_eq!(Solution::two_sum(vec![1, 3, 5], 8), vec![1, 2]);
        assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
        assert_eq!(Solution::two_sum(vec![3, 2, 4], 6), vec![1, 2]);
        assert_eq!(Solution::two_sum(vec![3, 3], 6), vec![0, 1]);
    }
}
