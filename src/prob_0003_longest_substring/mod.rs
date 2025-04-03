pub struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        if s.is_empty() {
            return 0;
        }

        let mut longest_substr = 1;
        let mut curr_substr_start_idx = 0;
        let mut curr_substr = &s[..=0];

        for (idx, c) in s.char_indices() {
            if curr_substr.contains(c) {
                while curr_substr_start_idx < idx {
                    if s.as_bytes()[curr_substr_start_idx] as char == c {
                        curr_substr_start_idx += 1;
                        break;
                    }
                    curr_substr_start_idx += 1;
                }
            }

            curr_substr = &s[curr_substr_start_idx..=idx];
            longest_substr = longest_substr.max(curr_substr.len());
        }

        longest_substr as i32
    }
}

#[cfg(test)]
mod tests {
    use crate::prob_0003_longest_substring::Solution;

    #[test]
    fn solution_works() {
        assert_eq!(
            Solution::length_of_longest_substring("abcabcbb".to_string()),
            3
        );
        assert_eq!(
            Solution::length_of_longest_substring("bbbbb".to_string()),
            1
        );
        assert_eq!(
            Solution::length_of_longest_substring("pwwkew".to_string()),
            3
        );
        assert_eq!(Solution::length_of_longest_substring(" ".to_string()), 1);
        assert_eq!(Solution::length_of_longest_substring("".to_string()), 0);
        assert_eq!(Solution::length_of_longest_substring("ab".to_string()), 2);
        assert_eq!(Solution::length_of_longest_substring("dvdf".to_string()), 3);
        assert_eq!(
            Solution::length_of_longest_substring("dvadf".to_string()),
            4
        );
        assert_eq!(
            Solution::length_of_longest_substring("anviaj".to_string()),
            5
        );
        assert_eq!(
            Solution::length_of_longest_substring("nfpdmpi".to_string()),
            5
        )
    }
}
