/**
 * [110] Balanced Binary Tree
 *
 * Given a binary tree, determine if it is height-balanced.
 *
 * For this problem, a height-balanced binary tree is defined as:
 *
 * <blockquote>
 * a binary tree in which the left and right subtrees of every node differ in height by no more than 1.
 * </blockquote>
 *
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/10/06/balance_1.jpg" style="width: 342px; height: 221px;" />
 *
 * Input: root = [3,9,20,null,null,15,7]
 * Output: true
 *
 *
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/10/06/balance_2.jpg" style="width: 452px; height: 301px;" />
 *
 * Input: root = [1,2,2,3,3,null,null,4,4]
 * Output: false
 *
 *
 * Example 3:
 *
 *
 * Input: root = []
 * Output: true
 *
 *
 *  
 * Constraints:
 *
 *
 * 	The number of nodes in the tree is in the range [0, 5000].
 * 	-10^4 <= Node.val <= 10^4
 *
 */
pub struct Solution {}
use crate::util::tree::{to_tree, TreeNode};

// problem: https://leetcode.com/problems/balanced-binary-tree/
// discuss: https://leetcode.com/problems/balanced-binary-tree/discuss/?currentPage=1&orderBy=most_votes&query=

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
    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        fn dfs(root: &Option<Rc<RefCell<TreeNode>>>) -> Option<i32> {
            if let Some(root) = root {
                let lhs = dfs(&root.borrow().left);
                let rhs = dfs(&root.borrow().right);
                match (lhs, rhs) {
                    (Some(l_depth), Some(r_depth)) => {
                        if (l_depth - r_depth).abs() > 1 {
                            None
                        } else {
                            Some(l_depth.max(r_depth) + 1)
                        }
                    }
                    _ => None,
                }
            } else {
                return Some(0);
            }
        }
        dfs(&root).is_some()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_110() {
        let tree = tree![1, 2, 2, 3, 3, null, null, 4, 4];
        assert_eq!(Solution::is_balanced(tree), false);
    }
}
