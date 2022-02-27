use std::str::FromStr;

pub struct Solution;

struct Complex {
    re: i32,
    im: i32,
}

impl FromStr for Complex {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let nums: Vec<&str> = s.split('+').collect();
        let re = nums[0].parse()?;
        let im = nums[1][..(nums[1].len() - 1)].parse()?;
        Ok(Self { im, re })
    }
}

impl std::ops::Mul for Complex {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            re: self.re * rhs.re - self.im * rhs.im,
            im: self.im * rhs.re + self.re * rhs.im,
        }
    }
}

impl ToString for Complex {
    fn to_string(&self) -> String {
        format!("{}+{}i", self.re, self.im)
    }
}

impl Solution {
    pub fn complex_number_multiply(num1: String, num2: String) -> String {
        let num1 = Complex::from_str(&num1).unwrap();
        let num2 = Complex::from_str(&num2).unwrap();
        (num1 * num2).to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn c1() {
        assert_eq!(
            Solution::complex_number_multiply("1+1i".to_owned(), "1+1i".to_owned()),
            "0+2i".to_owned()
        )
    }
}
