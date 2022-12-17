









// 112. Path Sum
// Binary Tree 18
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
    fn travel(root: Option<Rc<RefCell<TreeNode>>>, sum: &mut i32, target_sum: i32) -> bool {
        if root.is_none() {
            return false;
        }
        let node = root.unwrap();
        let inner = node.borrow();
        *sum += inner.val;
        if inner.left.is_none() && inner.right.is_none() && *sum == target_sum{
            return true;
        }
        if Self::travel(inner.left.clone(), sum, target_sum) || 
            Self::travel(inner.right.clone(), sum, target_sum){
            return true;
        }
        *sum -= inner.val;
        return false;
    }
    pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
        let mut sum = 0;
        return Self::travel(root, &mut sum, target_sum);
    }
}
fn main() {
}
