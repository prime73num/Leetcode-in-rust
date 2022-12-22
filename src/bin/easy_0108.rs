










// 108. Convert Sorted Array to Binary Search Tree
// Binary Tree 33


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
    fn trabel(nums: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
        if nums.is_empty() {
            return None;
        }
        let max = (0+nums.len())/2;
        let mut node = TreeNode::new(nums[max]);
        node.left = Self::trabel(&nums[0..max]);
        node.right = Self::trabel(&nums[max+1..]);
        Some(Rc::new(RefCell::new(node)))
    }

    pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        return Self::trabel(&nums);
    }
}
fn main() {
}
