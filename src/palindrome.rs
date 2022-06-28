pub struct Solution;

// Runtime: 4 ms, faster than 91.41% of Rust online submissions for Palindrome Number.
// Memory Usage: 2.1 MB, less than 68.89% of Rust online submissions for Palindrome Number.

#[allow(dead_code)]
impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        match x {
            x if x < 0 => false,
            x => {
                let mut num = x;
                let mut p = 0; // p will equal x if x is a palindrome

                while num > 0 {
                    p *= 10;
                    p += num % 10;
                    num /= 10;
                }
                p == x
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::palindrome::Solution;

    #[test]
    fn solution_works() {
        assert_eq!(Solution::is_palindrome(121), true);
        assert_eq!(Solution::is_palindrome(3), true);
        assert_eq!(Solution::is_palindrome(12321), true);
        assert_eq!(Solution::is_palindrome(-121), false);
    }
}
