







// 102. Binary Tree Level Order Traversal
// Binary Tree 5


#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
  pub val: i32,
  pub left: Option<Rc<RefCell<TreeNode>>>,
  pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
  #[inline]
  pub fn new(val: i32) -> Self {
    TreeNode {
      val,
      left: None,
      right: None
    }
  }
}
use std::rc::Rc;
use std::cell::RefCell;
struct Solution {}
impl Solution {
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut res = Vec::new();
        let mut row = Vec::new();
        let mut stack = std::collections::VecDeque::new();
        if root.is_none() { return res;}
        stack.push_back(root);
        stack.push_back(None);
        while !stack.is_empty() {
            let temp = stack.pop_front().unwrap();
            if let Some(node) = temp {
                let inner = node.borrow();
                if inner.left.is_some() {
                    stack.push_back(inner.left.clone());
                }
                if inner.right.is_some() {
                    stack.push_back(inner.right.clone());
                }
                row.push(inner.val);
            } else {
                res.push(row);
                row = Vec::new();
                if !stack.is_empty() {
                    stack.push_back(None);
                }
            }
        }
        res
    }
}

fn main() {
}
