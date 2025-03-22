pub struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        "foobar".to_string()
    }
}

#[cfg(test)]
mod tests {
    use crate::prob_0006_zigzag_conversation::Solution;

    #[test]
    fn solution_works() {
        assert_eq!(
            Solution::convert("PAYPALISHIRING".to_string(), 3),
            "PAHNAPLSIIGYIR".to_string()
        );
        assert_eq!(
            Solution::convert("PAYPALISHIRING".to_string(), 4),
            "PINALSIGYAHRPI".to_string()
        );
        assert_eq!(
            Solution::convert("A".to_string(), 1),
            "A".to_string()
        );
    }
}
