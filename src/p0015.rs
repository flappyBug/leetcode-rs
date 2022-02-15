// https://leetcode.com/problems/3sum/
use crate::utils::k_sum;

pub struct Solution;

impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.sort_unstable();
        k_sum(nums.as_ref(), 0, 3, 0)
    }
}
