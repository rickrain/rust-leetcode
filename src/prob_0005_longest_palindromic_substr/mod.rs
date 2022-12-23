pub struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        Self::find_longest_palindrome(&s).to_string()
    }

    fn find_longest_palindrome(s: &str) -> &str {
        let mut result = "";
        for (i, _c) in s.chars().enumerate() {
            let mut left_index: i32 = i as i32;
            let mut right_index = i;
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

            left_index = i as i32;
            right_index = i + 1;
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
        }
        result
    }

    #[allow(clippy::explicit_counter_loop)]
    fn is_palindrome(s: &str) -> bool {
        let mut forward_index = 0;
        for c in s.chars().rev() {
            if c != s.chars().nth(forward_index).unwrap_or_default() {
                return false;
            }
            forward_index += 1;
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use crate::prob_0005_longest_palindromic_substr::Solution;

    #[test]
    fn solution_works() {
        assert_eq!(Solution::is_palindrome("s"), true);
        assert_eq!(Solution::is_palindrome("aba"), true);
        assert_eq!(Solution::is_palindrome("aaaaa"), true);
        assert_eq!(Solution::is_palindrome("adada"), true);

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

// b a b a d d t a t t a r r a t t a t d d e t a r t r a t e e d r e d i v i d e r b
//   a b a d d t a t t a r r a t t a t d d e t a r t r a t e e d r e d i v i d e r
//     b a d d t a t t a r r a t t a t d d e t a r t r a t e e d r e d i v i d e
//       a d d t a t t a r r a t t a t d d e t a r t r a t e e d r e d i v i d
//         d d t a t t a r r a t t a t d d e t a r t r a t e e d r e d i v i
//           d t a t t a r r a t t a t d d e t a r t r a t e e d r e d i v
//             t a t t a r r a t t a t d d e t a r t r a t e e d r e d i
//               a t t a r r a t t a t d d e t a r t r a t e e d r e d
//                 t t a r r a t t a t d d e t a r t r a t e e d r e
//                   t a r r a t t a t d d e t a r t r a t e e d r
//                     a r r a t t a t d d e t a r t r a t e e d
//                       r r a t t a t d d e t a r t r a t e e
//                         r a t t a t d d e t a r t r a t e
//                           a t t a t d d e t a r t r a t
//                             t t a t d d e t a r t r a
//                               t a t d d e t a r t r
//                                 a t d d e t a r t
//                                   t d d e t a r
//                                     d d e t a
//                                       d e t
//                                         e
