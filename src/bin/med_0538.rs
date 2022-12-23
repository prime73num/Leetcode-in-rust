











// 538. Convert BST to Greater Tree
// Binary Tree 34
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
    fn travel(root: Option<Rc<RefCell<TreeNode>>>, sum: &mut i32) ->  Option<Rc<RefCell<TreeNode>>> {
        if root.is_none() { return None; }
        let node = root.unwrap();
        let inner = node.borrow();
        let right = Self::travel(inner.right.clone(), sum);
        *sum += inner.val;
        let mut new_root = TreeNode::new(*sum);
        new_root.right = right;
        let left = Self::travel(inner.left.clone(), sum);
        new_root.left = left;
        Some(Rc::new(RefCell::new(new_root)))
    }
    pub fn convert_bst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut sum = 0;
        return Self::travel(root, &mut sum);
    }
}
fn main() {
}
