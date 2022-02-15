// https://leetcode.com/problems/generate-parentheses/
pub struct Solution;

impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut ans = vec![];
        Solution::backtrack(&mut ans, String::new(), 0, 0, n as usize);
        ans
    }

    fn backtrack(ans: &mut Vec<String>, cur: String, open: usize, close: usize, max: usize) {
        if cur.len() == max * 2 {
            ans.push(cur);
            return;
        }

        if open < max {
            Solution::backtrack(ans, cur.clone() + "(", open + 1, close, max);
        }
        if close < open {
            Solution::backtrack(ans, cur + ")", open, close + 1, max);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test() {
        assert_eq!(
            Solution::generate_parenthesis(3),
            vec![
                "((()))".to_owned(),
                "(()())".to_owned(),
                "(())()".to_owned(),
                "()(())".to_owned(),
                "()()()".to_owned()
            ]
        )
    }
}
