pub struct Solution;

// Runtime: 3 ms, faster than 50.86% of Rust online submissions for Permutations.
// Memory Usage: 2.2 MB, less than 61.21% of Rust online submissions for Permutations.

#[allow(dead_code)]
impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        match nums.len() {
            1 => return vec![nums],
            _ => {
                // result is the outer-most vector
                let mut result: Vec<Vec<i32>> = vec![];

                for (index, x) in nums.iter().enumerate() {
                    // Create a new vector (remaining numbers) with the current value 'x' removed
                    let mut rem_nums = nums.clone();
                    rem_nums.remove(index);

                    // Recursively call permute() on the remaining numbers to create a new
                    // permutation. Merge the two permutations together.
                    for p in Solution::permute(rem_nums) {
                        result.push(vec![vec![*x], p].concat());
                    }
                }
                result
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::prob_0046_permutations::Solution;

    #[test]
    fn permute_works() {
        assert_eq!(Solution::permute(vec![1]), vec![vec![1]]);
        assert_eq!(Solution::permute(vec![1, 2]), vec![vec![1, 2], vec![2, 1]]);
        assert_eq!(
            Solution::permute(vec![1, 2, 3]),
            vec![
                vec![1, 2, 3],
                vec![1, 3, 2],
                vec![2, 1, 3],
                vec![2, 3, 1],
                vec![3, 1, 2],
                vec![3, 2, 1]
            ]
        );
    }
}
