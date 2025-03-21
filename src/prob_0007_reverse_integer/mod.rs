pub struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let x_str = x.to_string();
        let mut result = String::with_capacity(x_str.len() + 1);

        x_str.chars().rev().for_each(|c| {
            if c == '-' {
                result.insert(0, c);
            } else {
                result.push(c);
            }
        });

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
        assert_eq!(Solution::reverse(-2147483648), 0);
    }
}
