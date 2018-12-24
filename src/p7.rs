// https://leetcode.com/problems/reverse-integer/
pub struct Solution;

impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let neg = x < 0;
        let mut s: String = x.abs().to_string().chars().rev().collect();
        if neg {s.insert(0, '-')}
        s.parse().unwrap_or(0)
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
                    assert_eq!(expected, Solution::reverse(x));
                }
            )*
        }
    }

    parameterized! {
        pos: (123, 321),
        neg: (-123, -321),
        zero: (0, 0),
        bound_zero: (120, 21),
    }
}