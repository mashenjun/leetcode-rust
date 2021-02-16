/**
 * [2] Add Two Numbers
 *
 * You are given two non-empty linked lists representing two non-negative integers. The digits are stored in reverse order, and each of their nodes contains a single digit. Add the two numbers and return the sum as a linked list.
 * You may assume the two numbers do not contain any leading zero, except the number 0 itself.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/10/02/addtwonumber1.jpg" style="width: 483px; height: 342px;" />
 * Input: l1 = [2,4,3], l2 = [5,6,4]
 * Output: [7,0,8]
 * Explanation: 342 + 465 = 807.
 *
 * Example 2:
 *
 * Input: l1 = [0], l2 = [0]
 * Output: [0]
 *
 * Example 3:
 *
 * Input: l1 = [9,9,9,9,9,9,9], l2 = [9,9,9,9]
 * Output: [8,9,9,9,0,0,0,1]
 *
 *  
 * Constraints:
 *
 * 	The number of nodes in each linked list is in the range [1, 100].
 * 	0 <= Node.val <= 9
 * 	It is guaranteed that the list represents a number that does not have leading zeros.
 *
 */
pub struct Solution {}
use crate::util::linked_list::{to_list, ListNode};

// problem: https://leetcode.com/problems/add-two-numbers/
// discuss: https://leetcode.com/problems/add-two-numbers/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
//
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut result: Option<Box<ListNode>> = None;
        let mut tail = &mut result; // tail 一直指向next
        let mut t = (l1, l2, 0, 0); // (list1, list2, sum, carry)
        loop {
            t = match t {
                (None, None, _, carry) => {
                    if carry == 0 {
                        // 最终状态，没有额外要处理的，直接break
                        break;
                    } else {
                        (None, None, carry, 0)
                    }
                }
                (Some(list), None, _, carry) | (None, Some(list), _, carry) => {
                    if list.val + carry >= 10 {
                        (list.next, None, list.val + carry - 10, 1)
                    } else {
                        (list.next, None, list.val + carry, 0)
                    }
                }
                (Some(l1), Some(l2), _, carry) => {
                    if l1.val + l2.val + carry >= 10 {
                        (l1.next, l2.next, l1.val + l2.val + carry - 10, 1)
                    } else {
                        (l1.next, l2.next, l1.val + l2.val + carry, 0)
                    }
                }
            };
            // 设置tail等于设置上一个节点的next
            *tail = Some(Box::new(ListNode::new(t.2)));
            // 更新tail为新的next
            tail = &mut tail.as_mut()?.next;
        }
        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2() {
        let list1 = linked!(2, 4, 3);
        let list2 = linked!(5, 6, 4);
        let expect = linked!(7, 0, 8);
        assert_eq!(Solution::add_two_numbers(list1, list2), expect)
    }
}
