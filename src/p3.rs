// https://leetcode.com/problems/longest-substring-without-repeating-characters/
use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut map = HashMap::new();
        s.chars()
            .enumerate()
            // map.insert(k, v) will return None if k doesn't exist,
            // or pop Some(previous_v) if k exists
            .map(|(idx, c)| (idx as i32, map.insert(c, idx as i32)))
            .fold((0, 0), |(length, lo), (idx, op)| match op {
                Some(new_lo) if new_lo >= lo => (length.max(idx - new_lo), new_lo + 1),
                _ => (length.max(idx - lo + 1), lo),
            })
            .0
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    fn test(s: &str, expect: i32) {
        assert_eq!(Solution::length_of_longest_substring(s.to_owned()), expect)
    }

    #[test]
    fn test_empty_string() {
        test("", 0)
    }

    #[test]
    fn test_one_char() {
        test("c", 1)
    }

    #[test]
    fn test_repeat_one_char() {
        test("ccc", 1)
    }

    #[test]
    fn test_normal_case() {
        test("pwwkew", 3)
    }
}
