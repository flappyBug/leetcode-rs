// https://leetcode.com/problems/remove-duplicates-from-sorted-array-ii/
pub struct Solution;
impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0
        }
        let mut i = 0;
        let (mut record, mut count) = (nums[0], 0);
        for j in 0..nums.len() {
            if nums[j] == record {
                if  count < 2 {
                    count += 1;
                    nums[i] = nums[j];
                    i += 1;
                }
            } else {
                record = nums[j];
                count = 1;
                nums[i] = nums[j];
                i += 1;
            }
        }
        i as i32
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn c1 () {
        assert_eq!(Solution::remove_duplicates(&mut vec![1,1,1,2,2,3]), 5);
    }
}