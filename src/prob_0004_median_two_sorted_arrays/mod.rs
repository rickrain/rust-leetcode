use std::cmp::{max, min};

pub struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        // Make sure our first parameter is always the smallest of the two arrays
        if nums1.len() > nums2.len() {
            self::Solution::find_median_sorted_arrays(nums2, nums1)
        // If either list is empty, just return the median of the other list
        } else if nums1.is_empty() {
            self::Solution::find_median(nums2)
        } else if nums2.is_empty() {
            self::Solution::find_median(nums1)
        // Otherwise, calculate the median for the 2 sorted arrays
        } else {
            let mut low = 0;
            let mut high = nums2.len() - 1;

            while low <= high {
                // Partition each array
                let mut mid1 = (low + high) / 2;
                if mid1 > nums1.len() {
                    mid1 = nums1.len();
                }

                let mid2 = ((nums1.len() + nums2.len() + 1) / 2) - mid1;

                // Get the values on either side of the partition for the smaller array
                let l1 = if mid1 == 0           { i32::MIN } else { nums1[mid1 - 1] };
                let r1 = if mid1 >= nums1.len() { i32::MAX } else { nums1[mid1]     };

                // Get the values on either side of the partition for the larger array
                let l2 = if mid2 == 0           { i32::MIN } else { nums2[mid2 - 1] };
                let r2 = if mid2 >= nums2.len() { i32::MAX } else { nums2[mid2]     };

                // We're done partitioning the array and can calculate the median
                if l1 <= r2 && l2 <= r1 {
                    if (nums1.len() + nums2.len()) % 2 == 0 {
                        return (max(l1, l2) + min(r1, r2)) as f64 / 2_f64;
                    } else {
                        return max(l1, l2).into();
                    }
                } else if low == high {
                    return (l2 + r1) as f64 / 2_f64;
                } else if l2 > r1 {
                    low = mid1 + 1;
                } else {
                    high = mid1 - 1;
                }
            }

            0.0
        }
    }

    fn find_median(nums: Vec<i32>) -> f64 {
        if nums.len() % 2 == 0 {
            // Return the average of the two numbers in the middle of the array
            let idx1 = (nums.len() - 1) / 2;
            let idx2 = nums.len() / 2;

            (nums[idx1] + nums[idx2]) as f64 / 2_f64
        } else {
            // Return the number in the middle of the array
            nums[nums.len() / 2] as f64
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::prob_0004_median_two_sorted_arrays::Solution;

    #[test]
    fn solution_works() {
        assert_eq!(
            Solution::find_median_sorted_arrays(vec![1, 3], vec![2]),
            2.0
        );
        assert_eq!(
            Solution::find_median_sorted_arrays(vec![1, 2], vec![3, 4]),
            2.5
        );
        assert_eq!(
            Solution::find_median_sorted_arrays(
                vec![1, 12, 15, 26, 38],
                vec![2, 13, 17, 30, 45, 60]
            ),
            17.0
        );
        assert_eq!(
            Solution::find_median_sorted_arrays(vec![0, 0], vec![0, 0]),
            0.0
        );
        assert_eq!(
            Solution::find_median_sorted_arrays(vec![], vec![1, 2, 3, 4]),
            2.5
        );
        assert_eq!(
            Solution::find_median_sorted_arrays(vec![1], vec![2, 3, 4, 5, 6]),
            3.5
        );
    }
}
