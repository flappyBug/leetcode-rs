// https://leetcode.com/problems/longest-palindromic-substring/
pub struct Solution;

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        if s.is_empty() {
            return String::new();
        }
        // assume s only contains ascii chars.
        // use as_bytes to accelerate random access
        let v = s.as_bytes();
        let (start, end) = (0..v.len())
            .map(|i| {
                let len = Solution::expand_around_center(v, i);
                (i - (len - 1) / 2, i + len / 2)
            })
            .max_by_key(|(start, end)| end - start)
            .unwrap();
        String::from_utf8(v[start..=end].to_owned()).unwrap()
    }

    fn expand_around_center(v: &[u8], i: usize) -> usize {
        macro_rules! count {
            ($lft:expr, $rt:expr) => {
                (0..=$lft)
                    .rev()
                    .zip($rt..v.len())
                    .take_while(|&(l, r)| v[l] == v[r])
                    .count()
                    * 2
            };
        }
        let len1 = count!(i, i) - 1;
        let len2 = count!(i, i + 1);
        len1.max(len2)
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    macro_rules! parameterized {
        ($($name:ident: $value: expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (s, expected) = $value;
                    assert_eq!(expected.to_owned(), Solution::longest_palindrome(s.to_owned()));
                }
            )*
        }
    }

    parameterized! {
        empty: ("", ""),
        one_char: ("a", "a"),
        two_char: ("aa", "aa"),
        normal: ("babad", "aba"),
    }
}
