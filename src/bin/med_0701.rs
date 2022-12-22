










// 701. Insert into a Binary Search Tree
// Binary Tree 29
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
    pub fn insert_into_bst(mut root: Option<Rc<RefCell<TreeNode>>>, val: i32) -> Option<Rc<RefCell<TreeNode>>> {
        let mut pos = &mut root;
        while pos.is_none() {
            if pos.as_ref().unwrap().borrow().val > val {
                pos = &mut pos.as_mut().unwrap().borrow_mut().left;
            }
        }
        unimplemented!();
    }
}
fn main() {
}
