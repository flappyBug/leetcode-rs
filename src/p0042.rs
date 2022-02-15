// https://leetcode.com/problems/trapping-rain-water/
pub struct Solution;

impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        if height.len() < 3 {
            return 0;
        }
        let mut l = 0;
        let mut r = height.len() - 1;
        let mut total = 0;
        let mut current_height = height[l].min(height[r]);
        while l < r {
            if height[l] < height[r] {
                current_height = height[l].max(current_height);
                total += current_height - height[l];
                l += 1;
            } else {
                current_height = height[r].max(current_height);
                total += current_height - height[r];
                r -= 1;
            }
        }
        total
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn c1() {
        assert_eq!(Solution::trap(vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]), 6)
    }
}
