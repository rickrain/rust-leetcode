pub struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let index1: usize;
        let index2: usize;
        let row: usize;

        if s.len() % 2 == 0 {
            index2 = s.len() / 2;
            index1 = index2 - 1;
            row = s.len() / 2 - 1
        } else {
            index1 = s.len() / 2;
            index2 = index1;
            row = s.len() / 2;
        }

        Self::find_longest_palindrome(&s, index1, index2, row).to_string()
    }

    fn find_longest_palindrome(s: &String, index1: usize, index2: usize, row: usize) -> &str {
        if s.chars().nth(index1) == s.chars().nth(index2) {
            // Palindrome
            let result = &s[index1..index2 + 1];
            if row > 0 {
                let next_result = Self::find_longest_palindrome(s, index1 - 1, index2 + 1, row - 1);
                if next_result.len() > result.len() {
                    return next_result;
                }
            }
            result
        } else if row > 0 {
            let left_result = Self::find_longest_palindrome(s, index1, index1, row);
            let right_result = Self::find_longest_palindrome(s, index2, index2, row);
            if left_result.len() > right_result.len() {
                left_result
            } else {
                right_result
            }
        } else if index2 - index1 > 1
            && s[index1 + 1..index2 + 1]
                .chars()
                .all(|c| c == s.chars().nth(index2).unwrap_or_default())
        {
            &s[index1 + 1..index2 + 1]
        } else if index2 - index1 > 1
            && s[index1..index2]
                .chars()
                .all(|c| c == s.chars().nth(index1).unwrap_or_default())
        {
            &s[index1..index2]
        } else {
            &s[index1..index1 + 1]
        }
    }
}

// x a b a d a b      <-- Row 0
//   a b a d a        <-- Row 1
//     b a d          <-- Row 2
//       a            <-- Row 3

// x a b a a d a b    <-- Row 0
//   a b a a d a      <-- Row 1
//     b a a d        <-- Row 2
//       a a          <-- Row 3

// a b b              <-- Row 0
//   b                <-- Row 1

// a b b b b          <-- Row 0
//   b b b            <-- Row 1
//     b              <-- Row 2

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
            "aba".to_string()
        );
        assert_eq!(
            Solution::longest_palindrome("cbbd".to_string()),
            "bb".to_string()
        );
        assert_eq!(
            Solution::longest_palindrome("xabaadab".to_string()),
            "ada".to_string()
        );
        assert_eq!(
            Solution::longest_palindrome("abcda".to_string()),
            "c".to_string()
        );
        assert_eq!(
            Solution::longest_palindrome("babadada".to_string()),
            "adada".to_string()
        );
    }
}


// b a b a d a d a
//   a b a d a d
//     b a d a
//       a d