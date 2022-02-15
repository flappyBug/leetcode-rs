// https://leetcode.com/problems/permutations/
pub struct Solution;

const FAC_TABLE: [usize; 12] = [
    1, 1, 2, 6, 24, 120, 720, 5040, 40320, 362_880, 3_628_800, 39_916_800,
];

impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let len = nums.len();
        let mut res = vec![vec![0; len]; FAC_TABLE[len]];
        #[allow(clippy::needless_range_loop)]
        for i in 0..FAC_TABLE[len] {
            let mut used = vec![false; len];
            for j in (1..=len).rev() {
                let nth = i / FAC_TABLE[j - 1] % j;
                let idx = used
                    .iter()
                    .enumerate()
                    .filter(|(_, u)| !(**u))
                    .nth(nth)
                    .unwrap()
                    .0;
                res[i][len - j] = nums[idx];
                used[idx] = true;
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn c1() {
        let given = vec![1, 2, 3];
        let expect = [
            [1, 2, 3],
            [1, 3, 2],
            [2, 1, 3],
            [2, 3, 1],
            [3, 1, 2],
            [3, 2, 1],
        ];
        assert_eq!(Solution::permute(given), expect);
    }
}
