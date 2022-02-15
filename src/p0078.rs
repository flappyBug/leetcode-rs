pub struct Solution;

impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        subsets_slice(&nums[..])
    }
}

fn subsets_slice(nums: &[i32]) -> Vec<Vec<i32>> {
    let mut ans = vec![];
    if nums.is_empty() {
        ans.push(vec![]);
        ans
    } else {
        let mut tmp = subsets_slice(&nums[1..]);
        ans.append(&mut tmp.clone());
        for v in tmp.iter_mut() {
            v.push(nums[0]);
        }
        ans.append(&mut tmp);
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn c1() {
        assert_eq!(Solution::subsets(vec![]), vec![vec![]]);
    }

    #[test]
    fn c2() {
        assert_eq!(Solution::subsets(vec![1]), vec![vec![], vec![1]])
    }
}
