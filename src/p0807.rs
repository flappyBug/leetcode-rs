// https://leetcode.com/problems/max-increase-to-keep-city-skyline/

pub struct Solution;

impl Solution {
    pub fn max_increase_keeping_skyline(grid: Vec<Vec<i32>>) -> i32 {
        let nrows = grid.len();
        let ncols = grid[0].len();
        let rows_max: Vec<i32> = grid.iter().map(|v| *v.iter().max().unwrap()).collect();
        let cols_max: Vec<i32> = (0..ncols)
            .map(|c| (0..nrows).map(|r| grid[r][c]).max().unwrap())
            .collect();
        let mut ans = 0;
        for r in 0..nrows {
            #[allow(clippy::needless_range_loop)]
            for c in 0..ncols {
                ans += rows_max[r].min(cols_max[c]) - grid[r][c];
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn c1() {
        let grid = vec![
            vec![3, 0, 8, 4],
            vec![2, 4, 5, 7],
            vec![9, 2, 6, 3],
            vec![0, 3, 1, 0],
        ];
        assert_eq!(Solution::max_increase_keeping_skyline(grid), 35);
    }
}
