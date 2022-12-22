









// 617. Merge Two Binary Trees
// Binary Tree 22


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
    fn trabel(root1: Option<Rc<RefCell<TreeNode>>>, root2: Option<Rc<RefCell<TreeNode>>>) ->  Option<Rc<RefCell<TreeNode>>> {
        if root1.is_none() && root2.is_none() {
            return None;
        }
        let mut sum = 0;
        let mut root1_left = None;
        let mut root1_right = None;
        let mut root2_left = None;
        let mut root2_right = None;
        if let Some(node) = root1 {
            let inner = node.borrow();
            sum += inner.val;
            root1_left = inner.left.clone();
            root1_right = inner.right.clone();
        }
        if let Some(node) = root2 {
            let inner = node.borrow();
            sum += inner.val;
            root2_left = inner.left.clone();
            root2_right = inner.right.clone();
        }
        let mut node = TreeNode::new(sum);
        node.left = Self::trabel(root1_left, root2_left);
        node.right = Self::trabel(root1_right, root2_right);
        Some(Rc::new(RefCell::new(node)))
    }
    pub fn merge_trees(root1: Option<Rc<RefCell<TreeNode>>>, root2: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        return Self::trabel(root1, root2);
    }
}
fn main() {
}
