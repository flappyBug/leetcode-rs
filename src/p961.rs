pub struct Solution;

impl Solution {
    pub fn repeated_n_times(a: Vec<i32>) -> i32 {
        if a[0] == a[1] || a[0] == a[2] {
            a[0]
        } else if a[1] == a[2] {
            a[1]
        } else {
            a.windows(4)
                .find_map(|w| {
                    if w[0] == w[3] || w[1] == w[3] || w[2] == w[3] {
                        Some(w[3])
                    } else {
                        None
                    }
                })
                .unwrap()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    macro_rules! parameterized {
        ($($value: expr,)*) => {
            $(
                let (x, expected) = $value;
                assert_eq!(expected, Solution::repeated_n_times(x));
            )*
        }
    }

    #[test]
    fn test() {
        parameterized! {
            (vec![1, 2, 3, 3], 3),
            (vec![2,1,2,5,3,2], 2),
            (vec![5,1,5,2,5,3,5,4], 5),
        }
    }
}
