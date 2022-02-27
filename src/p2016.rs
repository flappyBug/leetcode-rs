pub struct Solution;

impl Solution {
    pub fn maximum_difference(nums: Vec<i32>) -> i32 {
        let mut min = nums[0];
        let mut diff = -1;
        for &num in nums[1..].iter() {
            min = min.min(num);
            diff = if num - min > 0 {
                (num - min).max(diff)
            } else {
                diff
            }
        }
        diff
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn c1() {
        assert_eq!(Solution::maximum_difference(vec![7, 1, 5, 4]), 4);
        assert_eq!(Solution::maximum_difference(vec![9, 4, 3, 2]), -1);
    }
}
