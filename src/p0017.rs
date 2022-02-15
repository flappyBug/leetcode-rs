pub struct Solution;

impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        let mut ans = vec![];
        for c in digits.chars() {
            ans = match c {
                '2' => combine(ans, vec!['a', 'b', 'c']),
                '3' => combine(ans, vec!['d', 'e', 'f']),
                '4' => combine(ans, vec!['g', 'h', 'i']),
                '5' => combine(ans, vec!['j', 'k', 'l']),
                '6' => combine(ans, vec!['m', 'n', 'o']),
                '7' => combine(ans, vec!['p', 'q', 'r', 's']),
                '8' => combine(ans, vec!['t', 'u', 'v']),
                '9' => combine(ans, vec!['w', 'x', 'y', 'z']),
                _ => return vec![],
            }
        }
        ans
    }
}

fn combine(head: Vec<String>, tail: Vec<char>) -> Vec<String> {
    if head.is_empty() {
        return tail.into_iter().map(|c| c.to_string()).collect();
    }
    let mut ans = vec![];
    for c in tail {
        let mut tmp = head.clone();
        for s in tmp.iter_mut() {
            s.push(c);
        }
        ans.append(&mut tmp);
    }
    println!("{:?}", ans);
    ans
}

#[cfg(test)]
mod tests {
    use super::Solution;

    macro_rules! parameterized {
        ($($name:ident: $value: expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (digits, expected) = $value;
                    assert_eq!(Solution::letter_combinations(digits.to_owned()), expected);
                }
            )*
        }
    }

    parameterized! {
        c1: ("23", vec!["ad", "bd", "cd", "ae", "be", "ce", "af", "bf", "cf"]),
        c2: ("", Vec::<String>::new()),
    }
}
