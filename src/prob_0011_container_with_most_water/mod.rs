use std::cmp;

pub struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut left_idx = 0;
        let mut right_idx = height.len() - 1;
        let mut max_area = 0;

        while left_idx < right_idx {
            let gap = (right_idx - left_idx) as i32;
            max_area = max_area.max(cmp::min(height[left_idx], height[right_idx]) * gap);

            if height[left_idx] < height[right_idx] {
                left_idx += 1;
            } else {
                right_idx -= 1;
            }
        }

        max_area
    }
}

#[cfg(test)]
mod tests {
    use crate::prob_0011_container_with_most_water::Solution;

    #[test]
    fn solution_works() {
        assert_eq!(Solution::max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]), 49);
        assert_eq!(Solution::max_area(vec![1, 1]), 1);
    }
}
