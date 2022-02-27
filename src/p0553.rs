pub struct Solution;

impl Solution {
    pub fn optimal_division(nums: Vec<i32>) -> String {
        let nums: Vec<String> = nums.iter().map(|num| num.to_string()).collect();
        match nums.len() {
            1 | 2 => nums.join("/"),
            _ => nums[0].to_string() + "/(" + &nums[1..].join("/") + ")",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn c1() {
        assert_eq!(
            Solution::optimal_division(vec![1000, 100, 10, 2]),
            "1000/(100/10/2)".to_owned()
        );
    }
}
