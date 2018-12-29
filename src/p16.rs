pub struct Solution;

impl Solution {
    pub fn three_sum_closest(mut nums: Vec<i32>, target: i32) -> i32 {
        nums.sort_unstable();
        let mut closest = nums[0] + nums[1] + nums[2];
        for i in 2..nums.len() {
            let (mut l, mut r) = (0, i - 1);
            while l < r {
                let three_sum = nums[l] + nums[r] + nums[i];
                if (three_sum - target).abs() < (closest - target).abs() {
                    closest = three_sum;
                }
                if three_sum > target {
                    while l + 1 < r && nums[r] == nums[r - 1] {
                        r -= 1;
                    }
                    r -= 1;
                } else if three_sum < target {
                    while l + 1 < r && nums[l] == nums[l + 1] {
                        l += 1;
                    }
                    l += 1;
                } else {
                    return target;
                }
            }
        }
        closest
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    macro_rules! parameterized {
        ($($name:ident: $value: expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (nums, target, expected) = $value;
                    assert_eq!(Solution::three_sum_closest(nums, target), expected);
                }
            )*
        }
    }

    parameterized! {
        c1: (vec![-1,2,1,-4], 1, 2),
        c2: (vec![1,1,1,1], 0, 3),
    }
}
