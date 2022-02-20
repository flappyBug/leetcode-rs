pub struct Solution;

impl Solution {
    pub fn is_one_bit_character(bits: Vec<i32>) -> bool {
        let mut unpair = false;
        let mut zero = true;
        for &bit in bits.iter() {
            match (unpair, bit) {
                (true, _) => unpair = false,
                (false, 0) => zero = true,
                (false, 1) => {
                    zero = false;
                    unpair = true
                }
                _ => unreachable!(),
            }
        }
        zero
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn c1() {
        let bits = vec![1, 0, 0];
        assert!(Solution::is_one_bit_character(bits));
    }

    #[test]
    fn c2() {
        let bits = vec![1, 1, 1, 0];
        assert!(!Solution::is_one_bit_character(bits));
    }
}
