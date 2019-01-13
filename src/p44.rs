// https://leetcode.com/problems/wildcard-matching/
pub struct Solution;
impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        let s = s.as_bytes();
        let p = p.as_bytes();
        let mut match_at = 0;
        let (mut s_idx, mut p_idx) = (0, 0);
        let mut star_idx = None;
        while s_idx < s.len() {
            if p_idx < p.len() && (s[s_idx] == p[p_idx] || p[p_idx] == b'?') {
                s_idx += 1;
                p_idx += 1;
            } else if p_idx < p.len() && p[p_idx] == b'*' {
                match_at = s_idx;
                star_idx = Some(p_idx);
                p_idx += 1;
            } else if star_idx.is_some() {
                p_idx = star_idx.unwrap() + 1;
                match_at += 1;
                s_idx = match_at;
            } else {
                return false;
            }
        }
        while p_idx < p.len() && p[p_idx] == b'*' {
            p_idx += 1;
        }
        p_idx == p.len()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    macro_rules! parameterized {
        ($($name:ident: $value:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (s, p, expected) = $value;
                    assert_eq!(Solution::is_match(s.to_owned(), p.to_owned()), expected);
                }
            )*
        }
    }

    parameterized! {
        c1: ("aa", "a", false),
        c2: ("aa", "*", true),
        c3: ("cb", "?a", false),
        c4: ("adceb", "*a*b", true),
        c5: ("acdcb", "a*c?b", false),
    }

}
