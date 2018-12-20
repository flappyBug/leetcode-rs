use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn two_sum(nums: &[i32], target: i32) -> Vec<i32> {
        let mut map = HashMap::new();
        for (idx, &num) in nums.iter().enumerate() {
            if let Some(&idy) = map.get(&num) {
                return vec![idy as i32, idx as i32];
            }
            map.insert(target - num, idx);
        }
        unreachable!()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_two_sum() {
        assert_eq!(Solution::two_sum(&vec![2, 7, 11, 15], 9), vec![0, 1])
    }
}