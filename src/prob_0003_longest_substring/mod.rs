use std::collections::HashMap;

pub struct Solution;


// abcabcbb
#[allow(dead_code)]
impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut char_map = HashMap::<char, i32>::new();
        let chars = s.chars();

        let mut substr_start = 0;
        let mut substr_end = 0;

        let mut max_substr_len = 0;
        let mut curr_substr_len = 0;

        for (i, c) in chars.enumerate() {
            let mut counter = char_map.entry(c).or_insert(0);
            *counter += 1;

            if *counter > 1 {
                if curr_substr_len > max_substr_len {
                    max_substr_len = curr_substr_len
                }
                
                // Walk-up substr_start until we find the current char 
                loop {
                    let start_char = s.chars().nth(substr_start).unwrap();
                    counter = char_map.get_mut(&start_char).unwrap();
                    *counter -= 1;
                    substr_start += 1;
                        
                    if start_char == c {
                        break;
                    }
                }

                substr_end = i + 1;
            } else {
                substr_end += 1;
            }
            
            curr_substr_len = substr_end - substr_start;
        }

        if curr_substr_len > max_substr_len {
            max_substr_len = curr_substr_len
        }

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
        assert_eq!(Solution::length_of_longest_substring("anviaj".to_string()), 5);
    }
}
