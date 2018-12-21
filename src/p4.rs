pub struct Solution;

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let (m, n) = (nums1.len(), nums2.len());
        if m > n {
            return Solution::find_median_sorted_arrays(nums2, nums1);
        }
        let (mut imin, mut imax, half_len) = (0, m, (m + n + 1) / 2);
        while imin <= imax {
            let i = (imin + imax) / 2;
            let j = half_len - i;
            if i < m && nums2[j - 1] > nums1[i] {
                imin = i + 1
            } else if i > 0 && nums1[i - 1] > nums2[j] {
                imax = i - 1
            } else {
                let max_of_left = if i == 0 {
                    nums2[j - 1]
                } else if j == 0 {
                    nums1[i - 1]
                } else {
                    nums1[i - 1].max(nums2[j - 1])
                } as f64;
                if (m + n) % 2 == 1 {
                    return max_of_left;
                }
                let min_of_right = if i == m {
                    nums2[j]
                } else if j == n {
                    nums1[i]
                } else {
                    nums1[i].min(nums2[j])
                } as f64;
                return (max_of_left + min_of_right) / 2.0;
            }
        }
        unreachable!()
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
                    let (nums1, nums2, expected) = $value;
                    assert_eq!(expected, Solution::find_median_sorted_arrays(nums1, nums2));
                }
            )*
        }
    }

    parameterized! {
        odd: (vec![1, 3], vec![2], 2.0),
        even:(vec![1, 2], vec![3, 4], 2.5),
        empty_left: (vec![], vec![1], 1.0),
        empty_right: (vec![1, 2], vec![], 1.5),
    }
}
