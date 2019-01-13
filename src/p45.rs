// https://leetcode.com/problems/jump-game-ii/
pub struct Solution;
impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        if nums.len() < 2 {
            return 0;
        }
        let mut jumped = 0;
        let mut current_idx = 0;
        let mut search_start = 1;
        let mut search_end = current_idx + nums[current_idx] as usize + 1; // exclusive end
        while search_end < nums.len() {
            // while can't finish in one jump
            current_idx = (search_start..search_end) // search range
                .into_iter()
                .max_by_key(|&idx| idx + nums[idx] as usize)
                .unwrap();
            search_start = search_end; // since position before search_end have been searched
            search_end = current_idx + nums[current_idx] as usize + 1;
            jumped += 1;
        }
        jumped + 1 // final jump
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn c1() {
        assert_eq!(Solution::jump(vec![2, 3, 1, 1, 4]), 2)
    }
}
