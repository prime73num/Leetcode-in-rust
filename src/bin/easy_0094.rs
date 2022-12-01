


// Binary Tree inorder Traversal






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
    fn travel(root: Option<&Rc<RefCell<TreeNode>>>, res: &mut Vec<i32>) {
        if root.is_none() { return; }
        let node = root.unwrap();
        let inner = node.borrow();
        Self::travel(inner.left.as_ref(), res);
        res.push(inner.val);
        Self::travel(inner.right.as_ref(), res);
    }
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut res = Vec::new();
        Self::travel(root.as_ref(), &mut res);
        res
    }
}

fn main() {
}
