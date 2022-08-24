pub struct Solution;

// Runtime: 0 ms, faster than 100.00% of Rust online submissions for Reverse Integer.
// Memory Usage: 2.1 MB, less than 67.57% of Rust online submissions for Reverse Integer.

#[allow(dead_code)]
impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let s = x.abs().to_string();
        let mut result = String::with_capacity(s.len() + 1);
        if x < 0 {
            result.push('-');
        }

        for c in s.chars().rev() {
            result.push(c);
        }
        result.parse::<i32>().unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use crate::prob_0007_reverse_integer::Solution;

    #[test]
    fn solution_works() {
        assert_eq!(Solution::reverse(123), 321);
        assert_eq!(Solution::reverse(-123), -321);
        assert_eq!(Solution::reverse(120), 21);
        assert_eq!(Solution::reverse(1534236469), 0);
        assert_eq!(Solution::reverse(0), 0);
    }
}
