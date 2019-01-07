pub struct Solution;

impl Solution {
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut counter = vec![0; candidates.len()];
        let mut res = vec![];
        backtrace(&candidates, &mut counter, 0, target, &mut res);
        res
    }
}

fn backtrace(
    candidates: &[i32],
    counter: &mut [i32],
    cursor: usize,
    remain: i32,
    res: &mut Vec<Vec<i32>>,
) {
    if cursor + 1 < candidates.len() {
        backtrace(candidates, counter, cursor + 1, remain, res);
    }
    if remain > candidates[cursor] {
        counter[cursor] += 1;
        backtrace(
            candidates,
            counter,
            cursor,
            remain - candidates[cursor],
            res,
        );
        counter[cursor] -= 1;
    } else if remain == candidates[cursor] {
        counter[cursor] += 1;
        res.push(output(candidates, counter));
        counter[cursor] -= 1;
    }
}

#[inline]
fn output(candidates: &[i32], counter: &[i32]) -> Vec<i32> {
    let mut res = Vec::with_capacity(counter.iter().sum::<i32>() as usize);
    for i in 0..counter.len() {
        if counter[i] != 0 {
            res.append(&mut vec![candidates[i]; counter[i] as usize]);
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_output() {
        assert_eq!(output(&[2, 3, 5], &[3, 2, 0]), [2, 2, 2, 3, 3]);
    }

    #[test]
    fn c1() {
        assert_eq!(
            Solution::combination_sum(vec![2, 3, 5], 8),
            vec![vec![3, 5], vec![2, 3, 3], vec![2, 2, 2, 2]]
        );
    }
}
