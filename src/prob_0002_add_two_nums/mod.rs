pub struct Solution;

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

#[allow(dead_code)]
impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        // Since we need to walk the lists, create 'mutable' copies
        let mut l1_current = l1;
        let mut l2_current = l2;

        // Carry and sum holders for each pair of nodes in the lists
        let mut carry = 0;
        let mut sum = 0;

        // Create the first node in the result list. This won't be returned in the final result.
        let mut result_head = Some(Box::new(ListNode::new(-1)));
        let mut result_current = result_head.as_mut();

        while (l1_current.is_some() || l2_current.is_some()) || carry > 0 {
            // Sum up the numbers plus carry (if applicable)
            if let Some(node) = l1_current {
                sum += node.val;
                l1_current = node.next;
            }

            if let Some(node) = l2_current {
                sum += node.val;
                l2_current = node.next
            }

            sum += carry;

            // Reset carry for next calculation
            if sum > 9 {
                carry = 1;
                sum -= 10;
            } else {
                carry = 0;
            }

            if let Some(node) = result_current {
                node.next = Some(Box::new(ListNode::new(sum)));
                result_current = node.next.as_mut();
            }

            // Reset sum to 0 for next calculation
            sum = 0;
        }

        result_head.unwrap().next
    }
}

#[cfg(test)]
mod tests {
    use crate::prob_0002_add_two_nums::Solution;

    use super::ListNode;

    #[test]
    fn solution_works() {
        let l1 = Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode {
                val: 4,
                next: Some(Box::new(ListNode::new(3))),
            })),
        }));
        let l2 = Some(Box::new(ListNode {
            val: 5,
            next: Some(Box::new(ListNode {
                val: 6,
                next: Some(Box::new(ListNode::new(4))),
            })),
        }));

        let l1_plus_l2 = Some(Box::new(ListNode {
            val: 7,
            next: Some(Box::new(ListNode {
                val: 0,
                next: Some(Box::new(ListNode::new(8))),
            })),
        }));

        assert_eq!(Solution::add_two_numbers(l1, l2), l1_plus_l2);

        let l1 = Some(Box::new(ListNode::new(0)));
        let l2 = Some(Box::new(ListNode::new(0)));
        let l1_plus_l2 = Some(Box::new(ListNode::new(0)));

        assert_eq!(Solution::add_two_numbers(l1, l2), l1_plus_l2);
    }
}
