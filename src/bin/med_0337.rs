// Dynamic Programming 31
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
    fn travel(root: Option<Rc<RefCell<TreeNode>>>, r: &mut i32, left: &mut i32, right: &mut i32) {
        if root.is_none() {
            *r = 0;
            *left = 0;
            *right = 0;
            return;
        }
        let node = root.unwrap();
        let inner = node.borrow();
        let mut sum_one = inner.val;
        let mut sum_two = 0;
        Self::travel(inner.left.clone(), r, left, right);
        let left_max = *r;
        sum_one += *left + *right;
        sum_two += *r;
        Self::travel(inner.right.clone(), r, left, right);
        let right_max = *r;
        sum_one += *left + *right;
        sum_two += *r;
        *left = left_max;
        *right = right_max;
        *r = sum_one;
        if sum_two > sum_one {
            *r = sum_two;
        }
    }
    pub fn rob(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut r = 0;
        let mut left = 0;
        let mut right = 0;
        Self::travel(root, &mut r, &mut left, &mut right);
        return r;
    }
}
fn main() {
}
