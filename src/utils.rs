// Definition for singly-linked list.
#[derive(PartialEq, Eq, Debug)]
pub struct ListNode<T> {
    pub val: T,
    pub next: Option<Box<ListNode<T>>>,
}

impl<T> ListNode<T> {
    #[inline]
    pub fn new(val: T) -> Self {
        ListNode { next: None, val }
    }
}

impl<T: Clone> Clone for ListNode<T> {
    fn clone(&self) -> Self {
        if let Some(ref next) = self.next {
            ListNode {
                val: self.val.clone(),
                next: Some(next.clone()),
            }
        } else {
            ListNode {
                val: self.val.clone(),
                next: None,
            }
        }
    }
}

#[derive(PartialEq, Eq, Debug)]
pub struct List<T>(pub Option<Box<ListNode<T>>>);

impl<T> List<T> {
    pub fn into_head(self) -> ListNode<T> {
        *self.0.unwrap()
    }
}

impl<T> From<Vec<T>> for List<T> {
    fn from(v: Vec<T>) -> Self {
        List(
            v.into_iter()
                .rev()
                .fold(None, |next, val| Some(Box::new(ListNode { val, next }))),
        )
    }
}

/// assume nums have been sorted
pub fn k_sum(nums: &[i32], target: i32, k: usize, index: usize) -> Vec<Vec<i32>> {
    let mut res = vec![];
    let len = nums.len();
    let (mut i, mut j) = (index, len - 1);
    if k == 2 {
        while i < j {
            if nums[i] == target - nums[j] {
                res.push(vec![nums[i], nums[j]]);
                while i < j && nums[i] == nums[i + 1] {
                    i += 1
                }
                while i < j && nums[j - 1] == nums[j] {
                    j -= 1
                }
                i += 1;
                j -= 1;
            } else if nums[i] < target - nums[j] {
                i += 1;
            } else {
                j -= 1;
            }
        }
    } else {
        loop {
            if i == len {
                break;
            }
            let mut tmp = k_sum(nums, target - nums[i], k - 1, i + 1);
            tmp.iter_mut().for_each(|v| v.push(nums[i]));
            res.append(tmp.as_mut());
            while i + 1 < len && nums[i] == nums[i + 1] {
                i += 1;
            }
            i += 1;
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_from_empty_vec() {
        let empty_lst: List<i32> = List(None);
        assert_eq!(List::from(vec![]), empty_lst);
    }

    #[test]
    fn test_list_from_vec() {
        assert_eq!(
            List::from(vec![1, 2]),
            List(Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode { val: 2, next: None })),
            })))
        )
    }
}
