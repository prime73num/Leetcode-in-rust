










// 101. Symmetric Tree
// Binary Tree 8


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
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let mut left_stack = vec![root.clone()];
        let mut right_stack = vec![root];
        while !left_stack.is_empty() {
            let temp_left = left_stack.pop().unwrap();
            let temp_right = right_stack.pop().unwrap();
            if temp_left.is_none() && temp_right.is_none() { continue; }
            if !(temp_left.is_some() && temp_right.is_some()) {return false;}

            let left_node = temp_left.unwrap();
            let right_node = temp_right.unwrap();

            let left_inner = left_node.borrow();
            let right_inner = right_node.borrow();
            if left_inner.val != right_inner.val {
                return false;
            }
            left_stack.push(left_inner.right.clone());
            left_stack.push(left_inner.left.clone());
            right_stack.push(right_inner.left.clone());
            right_stack.push(right_inner.right.clone());
        }
        true
    }
}
fn main() {
}
