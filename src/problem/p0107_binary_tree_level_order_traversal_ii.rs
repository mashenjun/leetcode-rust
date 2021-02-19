/**
 * [107] Binary Tree Level Order Traversal II
 *
 * Given a binary tree, return the bottom-up level order traversal of its nodes' values. (ie, from left to right, level by level from leaf to root).
 *
 *
 * For example:<br />
 * Given binary tree [3,9,20,null,null,15,7],<br />
 *
 *     3
 *    / \
 *   9  20
 *     /  \
 *    15   7
 *
 *
 *
 * return its bottom-up level order traversal as:<br />
 *
 * [
 *   [15,7],
 *   [9,20],
 *   [3]
 * ]
 *
 *
 */
pub struct Solution {}
use crate::util::tree::{to_tree, TreeNode};

// problem: https://leetcode.com/problems/binary-tree-level-order-traversal-ii/
// discuss: https://leetcode.com/problems/binary-tree-level-order-traversal-ii/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
//
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn level_order_bottom(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        use std::collections::VecDeque;

        let mut result: Vec<Vec<i32>> = vec![];
        let mut queue: Vec<Rc<RefCell<TreeNode>>> = vec![];
        match root {
            Some(node) => {
                queue.push(node);
            }
            None => return result,
        }
        while !queue.is_empty() {
            let mut next_level: Vec<Rc<RefCell<TreeNode>>> = vec![];
            let mut vals: Vec<i32> = vec![];
            queue.drain(..).for_each(|mut node| {
                vals.push(node.borrow().val);
                let mut node = node.borrow_mut();
                if let Some(left) = node.left.take() {
                    next_level.push(left)
                }
                if let Some(right) = node.right.take() {
                    next_level.push(right)
                }
            });
            // replace queue with next_level
            queue = next_level;
            result.push(vals);
        }
        result.reverse();
        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_107() {
        let expect: Vec<Vec<i32>> = vec![vec![15, 7], vec![9, 20], vec![3]];
        assert_eq!(
            Solution::level_order_bottom(tree![3, 9, 20, null, null, 15, 7]),
            expect
        );
    }
}
