// https://leetcode.com/problems/zigzag-conversion/
pub struct Solution;

impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        if num_rows < 2 {
            return s;
        }
        let sb = s.as_bytes();
        let num_rows = num_rows as usize;
        let chunk_size = num_rows * 2 - 2;
        let mut rows = Vec::with_capacity(num_rows);
        for i in 0..num_rows {
            let row:Vec<u8> = if i == 0 || i == num_rows - 1 {
                sb.iter().skip(i).step_by(chunk_size).cloned().collect()
            } else {
                let mut row = Vec::with_capacity(2 * sb.len() / chunk_size);
                let mut offset = 0;
                while offset + i < sb.len() {
                    row.push(sb[offset + i]);
                    offset += chunk_size;
                    if offset - i < sb.len() {
                        row.push(sb[offset - i]);
                    }
                }
                row
            };
            rows.push(row);
        }
        let bytes: Vec<_> = rows.into_iter().flatten().collect();
        String::from_utf8(bytes).unwrap()
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
    }
}