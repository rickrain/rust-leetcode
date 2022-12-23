pub struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        Self::find_longest_palindrome(&s).to_string()
    }
    fn expand_to_longest_palindrome(mut left_index: i32, mut right_index: usize, s: &str) -> &str {
        let mut result = "";
        while left_index >= 0
            && right_index < s.len()
            && s.chars().nth(left_index as usize) == s.chars().nth(right_index)
        {
            if (right_index - left_index as usize + 1) > result.len() {
                result = &s[left_index as usize..right_index + 1];
            }
            left_index -= 1;
            right_index += 1;
        }
        result
    }

    fn find_longest_palindrome(s: &str) -> &str {
        let mut result = "";

        // If all the chars are the same, then the entire string is a palindrome.
        if s.chars().eq(s.chars().rev()) {
            return s;
        }

        for (i, _c) in s.chars().enumerate() {
            // Expand outward from a single char
            let mut last_palindrome = Self::expand_to_longest_palindrome(i as i32, i as usize, s);
            if last_palindrome.len() > result.len() {
                result = last_palindrome;
            }

            // Expand outward from two adjacent chars
            last_palindrome = Self::expand_to_longest_palindrome(i as i32, i + 1_usize, s);
            if last_palindrome.len() > result.len() {
                result = last_palindrome;
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use crate::prob_0005_longest_palindromic_substr::Solution;

    #[test]
    fn solution_works() {
        assert_eq!(
            Solution::longest_palindrome("a".to_string()),
            "a".to_string()
        );
        assert_eq!(
            Solution::longest_palindrome("ac".to_string()),
            "a".to_string()
        );
        assert_eq!(
            Solution::longest_palindrome("aaa".to_string()),
            "aaa".to_string()
        );
        assert_eq!(
            Solution::longest_palindrome("abb".to_string()),
            "bb".to_string()
        );
        assert_eq!(
            Solution::longest_palindrome("aba".to_string()),
            "aba".to_string()
        );
        assert_eq!(
            Solution::longest_palindrome("xabadab".to_string()),
            "badab".to_string()
        );
        assert_eq!(
            Solution::longest_palindrome("babad".to_string()),
            "bab".to_string()
        );
        assert_eq!(
            Solution::longest_palindrome("cbbd".to_string()),
            "bb".to_string()
        );
        assert_eq!(
            Solution::longest_palindrome("xabaadab".to_string()),
            "aba".to_string()
        );
        assert_eq!(
            Solution::longest_palindrome("abcda".to_string()),
            "a".to_string()
        );
        assert_eq!(
            Solution::longest_palindrome("babadada".to_string()),
            "adada".to_string()
        );
        assert_eq!(
            Solution::longest_palindrome("babaddtattarrattatddetartrateedredividerb".to_string()),
            "ddtattarrattatdd".to_string()
        );
    }
}
