pub struct Solution;

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
    use crate::prob_0009_palindrome::Solution;

    #[test]
    fn solution_works() {
        assert_eq!(Solution::is_palindrome(121), true);
        assert_eq!(Solution::is_palindrome(3), true);
        assert_eq!(Solution::is_palindrome(12321), true);
        assert_eq!(Solution::is_palindrome(-121), false);
    }
}
