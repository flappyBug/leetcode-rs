// https://leetcode.com/problems/string-to-integer-atoi/submissions/
pub struct Solution;

#[derive(Copy, Clone)]
enum Sign {
    Pos,
    Neg,
}

impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        let slice = s.trim_start();

        let (sign, skip_count) = match slice.chars().next() {
            Some('-') => (Sign::Neg, 1),
            Some('+') => (Sign::Pos, 1),
            Some(c) if c.is_digit(10) => (Sign::Pos, 0),
            _ => return 0,
        };
        let mut num = 0;
        match sign {
            Sign::Pos => {
                for c in slice.chars().skip(skip_count) {
                    if let Some(digit) = c.to_digit(10) {
                        let digit = digit as i32;
                        if (i32::max_value() - digit) / 10 >= num {
                            num = num * 10 + digit;
                        } else {
                            return i32::max_value();
                        }
                    } else {
                        return num;
                    }
                }
            }
            Sign::Neg => {
                for c in slice.chars().skip(skip_count) {
                    if let Some(digit) = c.to_digit(10) {
                        let digit = digit as i32;
                        if (i32::min_value() + digit) / 10 <= num {
                            num = num * 10 - digit;
                        } else {
                            return i32::min_value();
                        }
                    } else {
                        return num;
                    }
                }
            }
        }
        num
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
                    assert_eq!(Solution::my_atoi(s.to_owned()), expected);
                }
            )*
        }
    }

    parameterized! {
        c1: ("42", 42),
        c2: ("   -42", -42),
        c3: ( "4193 with words", 4193),
        c4: ("words and 987", 0),
        c5: ("-91283472332", -2147483648),
    }
}
