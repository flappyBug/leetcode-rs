use std::str::FromStr;

pub struct Solution;

#[derive(Debug, Eq, PartialEq)]
enum Hir {
    Exact(u8),
    ZeroOrMore(Box<Hir>),
    Glob,
}

#[derive(Debug)]
struct Pattern(Vec<Hir>);

impl FromStr for Pattern {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        let mut exprs = vec![];
        let bytes = s.as_bytes();
        for &byte in bytes {
            match byte {
                b'.' => exprs.push(Hir::Glob),
                b'*' => match exprs.pop() {
                    None | Some(Hir::ZeroOrMore(_)) => return Err(()),
                    Some(hir) => exprs.push(Hir::ZeroOrMore(Box::new(hir))),
                },
                c => exprs.push(Hir::Exact(c)),
            }
        }
        Ok(Pattern(exprs))
    }
}

impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        let p: Pattern = p.parse().unwrap();
        Solution::is_match_recursive(s.as_bytes(), &p.0[..])
    }

    fn is_match_recursive(s: &[u8], p: &[Hir]) -> bool {
        if s.is_empty() && p.is_empty() {
            return true
        } else if p.is_empty() {
            return false
        }
        match p[0] {
            Hir::Exact(c) => {
                if s.is_empty() || s[0] != c {
                    return false
                }
                Solution::is_match_recursive(&s[1..], &p[1..])
            },
            Hir::Glob => {
                if s.is_empty() {
                    return false
                }
                Solution::is_match_recursive(&s[1..], &p[1..])
            },
            Hir::ZeroOrMore(ref h) => {
                if s.is_empty() {
                    return Solution::is_match_recursive(s, &p[1..])
                }
                match **h {
                    Hir::Exact(c) if s[0] != c => {
                        Solution::is_match_recursive(s, &p[1..])
                    }
                    _ => {
                        Solution::is_match_recursive(s, &p[1..]) ||
                            Solution::is_match_recursive(&s[1..], p)
                    }
                }
            }
        }
    }
}

// other's faster solution:
//impl Solution {
//    pub fn is_match(s: String, p: String) -> bool {
//        use crate::CharPattern::*;
//
//        let mut nfa = Vec::new();
//        let mut chars = p.as_bytes().iter().peekable();
//        while let Some(&c) = chars.next() {
//            let repeat = match chars.peek() {
//                Some(&&c2) if c2 == b'*' => {
//                    let _ = chars.next();
//                    true
//                }
//                _ => false,
//            };
//            let pat = match c {
//                b'.' => Any,
//                c => Exact(c),
//            };
//            nfa.push(State { pat, repeat });
//        }
//
//        if nfa.is_empty() {
//            return s.is_empty();
//        }
//
//        let chars = s.as_bytes();
//        let mut visited = std::collections::HashSet::new();
//        let mut states = std::collections::VecDeque::new();
//        states.push_back((0, 0));
//        while let Some(state) = states.pop_front() {
//            if visited.contains(&state) {
//                continue;
//            } else {
//                visited.insert(state);
//            }
//            let (state, i) = state;
//            if state == nfa.len() && i == chars.len() {
//                return true;
//            }
//            if state >= nfa.len() {
//                continue;
//            }
//            let s = &nfa[state];
//            if s.repeat {
//                states.push_back((state + 1, i));
//            }
//            if i < chars.len() && s.pat.matches(chars[i]) {
//                states.push_back((state + 1, i + 1));
//                if s.repeat {
//                    states.push_back((state, i + 1));
//                }
//            }
//        }
//        false
//    }
//}
//
//enum CharPattern {
//    Exact(u8),
//    Any
//}
//
//impl CharPattern {
//    fn matches(&self, c: u8) -> bool {
//        match self {
//            CharPattern::Exact(c2) if &c == c2 => true,
//            CharPattern::Any => true,
//            _ => false
//        }
//    }
//}
//
//struct State {
//    pat: CharPattern,
//    repeat: bool
//}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_fail() {
        let parse_fail = ["*", "f**"];
        assert_eq!(
            parse_fail
                .iter()
                .filter_map(|p| p.parse::<Pattern>().ok())
                .count(),
            0
        );
    }

    #[test]
    fn parse_success() {
        assert_eq!("".parse::<Pattern>().unwrap().0, Vec::new());
        assert_eq!("p".parse::<Pattern>().unwrap().0, vec![Hir::Exact(b'p')]);
        assert_eq!(".".parse::<Pattern>().unwrap().0, vec![Hir::Glob]);
        assert_eq!(
            ".*".parse::<Pattern>().unwrap().0,
            vec![Hir::ZeroOrMore(Box::new(Hir::Glob))]
        );
        assert_eq!(
            "p*".parse::<Pattern>().unwrap().0,
            vec![Hir::ZeroOrMore(Box::new(Hir::Exact(b'p')))]
        );
        assert_eq!(
            "p.c*.*".parse::<Pattern>().unwrap().0,
            vec![
                Hir::Exact(b'p'),
                Hir::Glob,
                Hir::ZeroOrMore(Box::new(Hir::Exact(b'c'))),
                Hir::ZeroOrMore(Box::new(Hir::Glob))
            ]
        )
    }

    #[test]
    fn c1() {
        assert!(!Solution::is_match("aa".to_owned(), "a".to_owned()));
        assert!(!Solution::is_match("mississippi".to_owned(), "mis*is*p*.".to_owned()));
        assert!(Solution::is_match("aa".to_owned(), "a*".to_owned()));
        assert!(Solution::is_match("aab".to_owned(), "c*a*b".to_owned()));
    }
}
