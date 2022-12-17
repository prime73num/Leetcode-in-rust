









// 110. Balanced Binary Tree
// Binary Tree 12


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
    fn travel(root: Option<Rc<RefCell<TreeNode>>>, depth: &mut i32) -> bool {
        if root.is_none() {
            *depth = 0;
            return true;
        }
        let node = root.unwrap();
        let mut left_depth = 0;
        if !Self::travel(node.borrow().left.clone(), &mut left_depth) {
            return false;
        }
        let mut right_depth = 0;
        if !Self::travel(node.borrow().right.clone(), &mut right_depth) {
            return false;
        }
        if left_depth > right_depth+1 || right_depth > left_depth+1 {
            return false;
        }
        if left_depth > right_depth {
            *depth = left_depth + 1;
        } else {
            *depth = right_depth + 1;
        }
        return true;
    }
    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let mut depth = 0;
        return Self::travel(root, &mut depth);
    }
}
fn main() {
}
