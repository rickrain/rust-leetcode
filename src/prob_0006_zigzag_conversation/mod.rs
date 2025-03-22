use std::ops::Range;

pub struct Solution;

const INCREASING: i32 = 1;
const DECREASING: i32 = -1;

struct Oscillator {
    range: Range<i32>,
    current: i32,
    direction: i32,
}

impl Oscillator {
    fn new(range: Range<i32>) -> Self {
        Oscillator {
            range: range.clone(),
            current: range.start,
            direction: INCREASING,
        }
    }
}

impl Iterator for Oscillator {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        let x = self.current;

        if self.range.start == self.range.end-1 {
            return Some(x);
        } else if self.current == self.range.start {
            self.direction = INCREASING;
        } else if self.current == self.range.end-1 {
            self.direction = DECREASING;
        }

        self.current += self.direction;

        Some(x)
    }
}

#[allow(dead_code)]
impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        let mut v = vec![String::new(); num_rows as usize];
        let s_bytes = s.as_bytes();

        let osc = Oscillator::new(0..num_rows);
        let osc_vals: Vec<i32> = osc.take(s.len()).collect();

        for osc_val in osc_vals.iter().enumerate() {
            v[*osc_val.1 as usize].push(char::from(s_bytes[osc_val.0]));
        }

        let mut result = String::new();
        for line in v {
            result.push_str(&line);
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use crate::prob_0006_zigzag_conversation::{Oscillator, Solution};

    #[test]
    fn oscillator_works() {
        let osc = Oscillator::new(0..4);
        let osc_vals: Vec<i32> = osc.take(7).collect();
        assert_eq!(osc_vals, vec![0, 1, 2, 3, 2, 1, 0]);

        let osc = Oscillator::new(-2..2);
        let osc_vals: Vec<i32> = osc.take(12).collect();
        assert_eq!(osc_vals, vec![-2, -1, 0, 1, 0, -1, -2, -1, 0, 1, 0, -1]);
    }

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
        assert_eq!(Solution::convert("A".to_string(), 1), "A".to_string());
        assert_eq!(Solution::convert("AB".to_string(), 1), "AB".to_string());
    }
}