// https://leetcode.com/problems/palindrome-number/submissions/
pub struct Solution;

impl Solution {
    pub fn is_palindrome(mut x: i32) -> bool {
        if x < 0 || (x % 10 == 0 && x != 0 ) { return false; }
        let mut reverted = 0;
        while x > reverted {
            reverted = reverted * 10 + x % 10;
            x /= 10;
        }
        x == reverted || x == reverted / 10
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
                    assert_eq!(expected, Solution::is_palindrome(x));
                }
            )*
        }
    }

    parameterized! {
        zero: (0, true),
        neg: (-1, false),
        ends_with_zero: (10, false),
        normal_case: (121, true),
    }
}