










// 513. Find Bottom Left Tree Value 
// Binary Tree 17
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
    pub fn find_bottom_left_value(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut queue = std::collections::VecDeque::new();
        queue.push_back(root);
        let mut res = 0;
        while !queue.is_empty() {
            let size = queue.len();
            for _ in 0..size {
                let temp = queue.pop_front().unwrap();
                if let Some(node) = temp {
                    let inner = node.borrow();
                    res = inner.val;
                    queue.push_back(inner.right.clone());
                    queue.push_back(inner.left.clone());
                }
            }
        }
        res
    }
}
fn main() {
}
