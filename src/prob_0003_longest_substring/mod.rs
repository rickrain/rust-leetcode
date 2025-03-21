use std::{char, collections::HashMap};

pub struct Solution;

// abcabcbb
#[allow(dead_code)]
impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut char_map: HashMap<char, i32> = HashMap::with_capacity(s.len() / 2);
        let chars = s.chars();

        let mut substr_start = 0;
        let mut substr_end = 0;

        let mut max_substr_len = 0;
        let mut curr_substr_len = 0;

        let s_bytes = s.as_bytes();

        for (i, c) in chars.enumerate() {
            let mut counter = char_map.entry(c).or_insert(0);
            *counter += 1;

            if *counter > 1 {
                max_substr_len = std::cmp::max(curr_substr_len, max_substr_len);

                // Walk-up substr_start until we find the current char
                loop {
                    let start_char = &s_bytes[substr_start];
                    let x = char::from(*start_char);

                    counter = char_map.get_mut(&x).unwrap();
                    *counter -= 1;
                    substr_start += 1;

                    if x == c {
                        break;
                    }
                }

                substr_end = i + 1;
            } else {
                substr_end += 1;
            }

            curr_substr_len = substr_end - substr_start;
        }

        max_substr_len = std::cmp::max(curr_substr_len, max_substr_len);

        max_substr_len as i32
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
            Solution::length_of_longest_substring("anviaj".to_string()),
            5
        );
    }
}
