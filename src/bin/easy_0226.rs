







// 226. Invert Binary Tree
// Binary Tree 6
//
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
    fn invert(root: Option<Rc<RefCell<TreeNode>>>) {
        if root.is_none() {return;}
        let node = root.unwrap();
        let mut inner = node.borrow_mut();
        Self::invert(inner.left.clone());
        Self::invert(inner.right.clone());
        let temp_left = inner.left.clone();
        let temp_right = inner.right.clone();
        inner.left = temp_right;
        inner.right = temp_left;
    }
    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        let res = root.clone();
        Self::invert(root);
        res
    }
}

fn main() {
}
