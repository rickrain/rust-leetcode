pub struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        let num_str = s.trim();

        // This will be the result'ing String to try to parse to i32 at the end.
        let mut result = String::with_capacity(num_str.len());

        // Get an iterator for the characters
        let mut num_str_chars = num_str.chars();

        // Store the sign if applicable. If first char is not a + or ', then
        // check if is a digit and if so, append it to the result, otherwise
        // we already know it's not valid and will return 0.
        let sign = match num_str_chars.next() {
            Some(c) if c == '+' || c == '-' => Some(c),
            Some(x) if x.is_ascii_digit() => {
                result.push(x);
                None
            }
            _ => return 0,
        };

        // Loop through original string until a non-digit is found or
        // end of string is reached
        for c in num_str_chars {
            if c.is_ascii_digit() {
                result.push(c);
            } else {
                break;
            }
        }

        // Trim leading zeros from the number
        result = result.trim_start_matches('0').to_string();

        // If result is more than 10 digits, then we know it's out
        // of the i32 MIN and MAX range.
        if result.len() > 10 {
            match sign {
                Some('-') => i32::MIN,
                _ => i32::MAX,
            }
        } else {
            if sign == Some('-') {
                result.insert(0, sign.unwrap());
            }
            let x = result
                .parse::<i64>()
                .unwrap_or_default()
                .clamp(i32::MIN as i64, i32::MAX as i64);
            x as i32
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::prob_0008_string_to_int::Solution;

    #[test]
    fn solution_works() {
        assert_eq!(Solution::my_atoi("42".to_string()), 42);
        assert_eq!(Solution::my_atoi(".1".to_string()), 0);
        assert_eq!(Solution::my_atoi("   -42".to_string()), -42);
        assert_eq!(Solution::my_atoi("4193 with words".to_string()), 4193);
        assert_eq!(Solution::my_atoi("dadjd".to_string()), 0);
        assert_eq!(Solution::my_atoi("-2147483648".to_string()), -2147483648);
        assert_eq!(Solution::my_atoi("-91283472332".to_string()), -2147483648);
        assert_eq!(
            Solution::my_atoi("20000000000000000000".to_string()),
            2147483647
        );
        assert_eq!(
            Solution::my_atoi("1234567890123456789012345678901234567890".to_string()),
            2147483647
        );
    }
}
