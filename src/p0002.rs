// https://leetcode.com/problems/add-two-numbers/
pub struct Solution;

type ListNode = crate::utils::ListNode<i32>;

impl Solution {
    pub fn add_two_numbers(
        l1: &Option<Box<ListNode>>,
        l2: &Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        Solution::add_two_numbers_with_carry(l1, l2, false)
    }

    fn add_two_numbers_with_carry(
        l1: &Option<Box<ListNode>>,
        l2: &Option<Box<ListNode>>,
        carry: bool,
    ) -> Option<Box<ListNode>> {
        match (l1, l2) {
            (Some(node1), Some(node2)) => {
                let v1 = node1.val;
                let v2 = node2.val;
                let v = v1 + v2 + carry as i32;
                Some(Box::new(ListNode {
                    val: v % 10,
                    next: Solution::add_two_numbers_with_carry(&node1.next, &node2.next, v > 9),
                }))
            }
            (Some(node), None) => {
                let v1 = node.val;
                let v = v1 + carry as i32;
                Some(Box::new(ListNode {
                    val: v % 10,
                    next: Solution::add_two_numbers_with_carry(&node.next, l2, v > 9),
                }))
            }
            (None, Some(_)) => Solution::add_two_numbers_with_carry(l2, l1, carry),
            (None, None) => {
                if carry {
                    Some(Box::new(ListNode::new(1)))
                } else {
                    None
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::utils::List;

    #[test]
    fn test_add_two_numbers() {
        let num1 = List::from(vec![2, 4, 3]).0;
        let num2 = List::from(vec![5, 6, 4]).0;
        let ans = List::from(vec![7, 0, 8]).0;
        assert_eq!(Solution::add_two_numbers(&num1, &num2), ans);
    }

    #[test]
    fn test_carry() {
        let num1 = List::from(vec![5]).0;
        let num2 = num1.clone();
        let ans = List::from(vec![0, 1]).0;
        assert_eq!(Solution::add_two_numbers(&num1, &num2), ans);
    }
}
