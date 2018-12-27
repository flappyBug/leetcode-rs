// https://leetcode.com/problems/valid-parentheses/submissions/
pub struct Solution;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut v = vec![];
        for c in s.chars() {
            match c {
                '(' | '[' | '{' => v.push(c),
                _ => match (v.pop(), c) {
                    (Some('('), ')') | (Some('['), ']') | (Some('{'), '}') => (),
                    _ => return false,
                },
            }
        }
        v.is_empty()
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
                    let (x, expected) = $value;
                    assert_eq!(expected, Solution::is_valid(x.to_owned()));
                }
            )*
        }
    }

    parameterized! {
        c1: ("()", true),
        c2: ("()[]{}", true),
        c3: ("(]", false),
        c4: ("([)]", false),
        c5: ("{[]}", true),
    }
}
