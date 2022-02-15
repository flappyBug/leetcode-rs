// https://leetcode.com/problems/remove-duplicates-from-sorted-array
pub struct Solution;

impl Solution {
    #[allow(clippy::ptr_arg)]
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }
        let mut i = 0;
        for j in 0..nums.len() {
            if nums[i] < nums[j] {
                i += 1;
                nums[i] = nums[j];
            }
        }
        i as i32 + 1

        // or use built-in method:
        // nums.dedup(); nums.len() as i32;
    }
}
