// https://leetcode.com/problems/first-missing-positive/
pub struct Solution;
// O(n) in time, O(1) in space.
impl Solution {
    pub fn first_missing_positive(mut nums: Vec<i32>) -> i32 {
        // For a given vector, traverse twice.
        // In first traverse, set `nums[num-1] = num` recursively if
        // num occurred in origin vector.
        // In second traverse, find first index (as `idx`) that doesn't
        // satisfy `nums[idx] == idx + 1`.
        // If found, the anser is `idx + 1`. `nums.len() + 1` in other case.
        let len = nums.len();
        for i in 0..len {
            if nums[i] <= 0 {
                // skip this case so we can convert from i32 to usize safely
                continue;
            }
            let mut num = nums[i] as usize;
            // while num should be recorded in place but didn't
            while num <= len && nums[num - 1] != num as i32 {
                if nums[num - 1] <= 0 || nums[num - 1] > len as i32 {
                    nums[num - 1] = num as i32;
                    break;
                } else {
                    let tmp = nums[num - 1];
                    nums[num - 1] = num as i32;
                    num = tmp as usize;
                }
            }
        }
        if let Some((idx, _)) = nums
            .into_iter()
            .enumerate()
            .find(|(idx, num)| *idx as i32 != num - 1)
        {
            1 + idx as i32
        } else {
            1 + len as i32
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn c1() {
        assert_eq!(Solution::first_missing_positive(vec![1, 2, 0]), 3);
        assert_eq!(Solution::first_missing_positive(vec![3, 4, -1, 1]), 2);
        assert_eq!(Solution::first_missing_positive(vec![7, 8, 9, 11, 12]), 1);
        assert_eq!(Solution::first_missing_positive(vec![1, 1]), 2);
    }
}
