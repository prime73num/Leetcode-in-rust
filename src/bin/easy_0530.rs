// 530. Minimum Absolute Difference in BST
// Binary Tree 25
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
    fn travel(root: Option<Rc<RefCell<TreeNode>>>, val: &mut i32, res: &mut i32) {
        if root.is_none() {
            return;
        }
        let node = root.unwrap();
        let inner = node.borrow();
        Self::travel(inner.left.clone(), val, res);
        let temp = inner.val as i64- *val as i64 ;
        if *res as i64 > temp {
            *res = temp as i32;
        }
        *val = inner.val;
        Self::travel(inner.right.clone(), val, res);
        return;
    }
    pub fn get_minimum_difference(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut val = std::i32::MIN;
        let mut res = std::i32::MAX;
        Self::travel(root, &mut val, &mut res);
        return res;
    }
}
fn main() {
}
