// https://leetcode.com/problems/group-anagrams/
pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        // My origin solution
        let mut sorted_bytes: Vec<Vec<u8>> = strs.iter().map(|s| s.as_bytes().to_owned()).collect();
        sorted_bytes.iter_mut().for_each(|v| v.sort_unstable());
        let mut map = HashMap::<Vec<u8>, Vec<usize>>::new();
        for (idx, bytes) in sorted_bytes.into_iter().enumerate() {
            map.entry(bytes).or_insert_with(Vec::new).push(idx)
        }
        map.values()
            .map(|idxs| {
                idxs.iter()
                    .map(|&idx| strs[idx].clone())
                    .collect::<Vec<String>>()
            })
            .collect()

        // Other's better solution

        // strs.into_iter()
        //     .fold(HashMap::new(), |mut map, s| {
        //         map.entry(s.bytes().fold([0; 26], |mut hash, b| {
        //             hash[(b - b'a') as usize] += 1u8;
        //             hash
        //         }))
        //         .or_insert(vec![])
        //         .push(s);
        //         map
        //     })
        //     .into_iter()
        //     .map(|s| s.1)
        //     .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn c1() {
        let given = vec![
            "eat".to_string(),
            "tea".to_string(),
            "tan".to_string(),
            "ate".to_string(),
            "nat".to_string(),
            "bat".to_string(),
        ];
        assert_eq!(Solution::group_anagrams(given).len(), 3);
    }
}
