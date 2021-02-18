/**
 * [94] Binary Tree Inorder Traversal
 *
 * Given the root of a binary tree, return the inorder traversal of its nodes' values.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/09/15/inorder_1.jpg" style="width: 202px; height: 324px;" />
 * Input: root = [1,null,2,3]
 * Output: [1,3,2]
 *
 * Example 2:
 *
 * Input: root = []
 * Output: []
 *
 * Example 3:
 *
 * Input: root = [1]
 * Output: [1]
 *
 * Example 4:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/09/15/inorder_5.jpg" style="width: 202px; height: 202px;" />
 * Input: root = [1,2]
 * Output: [2,1]
 *
 * Example 5:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/09/15/inorder_4.jpg" style="width: 202px; height: 202px;" />
 * Input: root = [1,null,2]
 * Output: [1,2]
 *
 *  
 * Constraints:
 *
 * 	The number of nodes in the tree is in the range [0, 100].
 * 	-100 <= Node.val <= 100
 *
 *  
 * Follow up:
 * Recursive solution is trivial, could you do it iteratively?
 *  
 *
 */
pub struct Solution {}
use crate::util::tree::{to_tree, TreeNode};

// problem: https://leetcode.com/problems/binary-tree-inorder-traversal/
// discuss: https://leetcode.com/problems/binary-tree-inorder-traversal/discuss/?currentPage=1&orderBy=most_votes&query=

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
    // inorder_traversal_1 is in recursive way.
    pub fn inorder_traversal_1(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        // inorder is a help function to implement traversal in recursive way.
        fn inorder(node: &TreeNode, result: &mut Vec<i32>) {
            if let Some(ref left) = node.left {
                inorder(&left.borrow(), result)
            }
            result.push(node.val);
            if let Some(ref right) = node.right {
                inorder(&right.borrow(), result);
            }
        }
        if let Some(node) = root {
            let mut result: Vec<i32> = vec![];
            inorder(&node.borrow(), &mut result);
            return result;
        }
        vec![]
    }

    // inorder_traversal_2 is in iteration way.
    pub fn inorder_traversal_2(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut result: Vec<i32> = vec![];
        let mut stack: Vec<Rc<RefCell<TreeNode>>> = vec![];
        let mut root = root;
        while root.is_some() || !stack.is_empty() {
            loop {
                if let Some(node) = root {
                    stack.push(node.clone());
                    root = node.borrow_mut().left.take();
                } else {
                    break;
                }
            }
            if let Some(node) = stack.pop() {
                result.push(node.borrow().val);
                root = node.borrow_mut().right.take();
            }
        }
        result
    }

    // reference to https://leetcode-cn.com/problems/binary-tree-inorder-traversal/solution/fei-di-gui-jie-fa-ke-yi-sui-yi-diao-zheng-cheng-qi/
    pub fn inorder_traversal_3(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut result: Vec<i32> = vec![];
        let mut stack: Vec<(Option<Rc<RefCell<TreeNode>>>, bool)> = vec![];
        stack.push((root, false));
        while !stack.is_empty() {
            if let Some((node, visited)) = stack.pop() {
                if let Some(node) = node {
                    if visited {
                        result.push(node.borrow().val);
                    } else {
                        // 此处从下到上是中序遍历的顺序
                        // 前序遍历把中间那条放最后
                        // 后续遍历把中间那条放最前
                        stack.push((node.borrow_mut().right.take(), false));
                        stack.push((Some(node.clone()), true));
                        stack.push((node.borrow_mut().left.take(), false))
                    }
                }
            }
        }
        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_94() {
        let expect: Vec<i32> = vec![1, 3, 2];
        assert_eq!(Solution::inorder_traversal_1(tree![1, null, 2, 3]), expect);
        assert_eq!(Solution::inorder_traversal_2(tree![1, null, 2, 3]), expect);
        assert_eq!(Solution::inorder_traversal_3(tree![1, null, 2, 3]), expect);
    }
}
