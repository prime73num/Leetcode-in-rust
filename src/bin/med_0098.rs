










// 98. Validate Binary Search Tree
// Binary Tree 24

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
    fn travel(root: Option<Rc<RefCell<TreeNode>>>, val: &mut i64) -> bool {
        if root.is_none() {
            return true;
        }
        let node = root.unwrap();
        let inner = node.borrow();
        if !Self::travel(inner.left.clone(), val) {
            return false;
        }
        if !(*val < inner.val as i64) {
            return false;
        }
        *val = inner.val as i64;
        if !Self::travel(inner.right.clone(), val) {
            return false;
        }
        return true;
    }
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let mut val = std::i64::MIN;
        return Self::travel(root, &mut val);
    }
}
fn main() {
}
        
   
