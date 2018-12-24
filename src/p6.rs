// https://leetcode.com/problems/zigzag-conversion/
pub struct Solution;

impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        if num_rows < 2 { return s; }
        let sb = s.as_bytes();
        let len = sb.len();
        let num_rows = num_rows as usize;
        let chunk_size = num_rows * 2 - 2;
        let mut buf = String::with_capacity(len);
        for i in 0..num_rows {
            for j in (0..).step_by(chunk_size).take_while(|&j| i + j < len) {
                buf.push(sb[i + j] as char);
                if i != 0 && i != num_rows - 1 && j + chunk_size - i < len {
                    buf.push(sb[j + chunk_size - i] as char);
                }
            }
        }
        buf
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
                    let (s, num_rows, expected) = $value;
                    assert_eq!(expected.to_owned(), Solution::convert(s.to_owned(), num_rows));
                }
            )*
        }
    }
    parameterized! {
        three_lines: ("LEETCODEISHIRING", 3, "LCIRETOESIIGEDHN"),
        four_lines: ("LEETCODEISHIRING", 4, "LDREOEIIECIHNTSG"),
        single_char: ("A", 3, "A"),
    }
}