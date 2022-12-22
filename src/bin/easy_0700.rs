




// 700. Search in a Binary Search Tree
// Binary Tree 23
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
    pub fn search_bst(root: Option<Rc<RefCell<TreeNode>>>, val: i32) -> Option<Rc<RefCell<TreeNode>>> {
        let mut res = root;
        while res.is_some() {
            let inner = res.as_ref().unwrap().borrow();
            if inner.val == val {
                drop(inner);
                return res;
            }
            let mut temp = None;
            if val < inner.val {
                temp = inner.left.clone();
            } else {
                temp = inner.right.clone();
            }
            drop(inner);
            res = temp;
        }
        return res;
    }
}
fn main() {
}
