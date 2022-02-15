// https://leetcode.com/problems/4sum/
use crate::utils::k_sum;
pub struct Solution;

impl Solution {
    pub fn four_sum(mut nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        nums.sort_unstable();
        k_sum(nums.as_ref(), target, 3, 0)
    }
}
