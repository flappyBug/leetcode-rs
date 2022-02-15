// https://leetcode.com/problems/spiral-matrix-ii/comments/
pub struct Solution;
impl Solution {
    pub fn generate_matrix(n: i32) -> Vec<Vec<i32>> {
        let n = n as usize;
        let mut res = vec![vec![0; n]; n];
        let mut s = 0;
        let mut e = n - 1;
        let mut num = 1;
        while s < e {
            for j in s..=e {
                res[s][j] = num;
                num += 1;
            }
            #[allow(clippy::needless_range_loop)]
            for i in (s + 1)..=e {
                res[i][e] = num;
                num += 1;
            }
            for j in (s..e).rev() {
                res[e][j] = num;
                num += 1;
            }
            for i in ((s + 1)..e).rev() {
                res[i][s] = num;
                num += 1;
            }
            s += 1;
            e -= 1;
        }
        if s == e {
            res[s][s] = num;
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn c1() {
        assert_eq!(
            Solution::generate_matrix(3),
            vec![vec![1, 2, 3], vec![8, 9, 4], vec![7, 6, 5]]
        );
    }

    #[test]
    fn c2() {
        assert_eq!(Solution::generate_matrix(1), vec![vec![1]]);
    }
}
