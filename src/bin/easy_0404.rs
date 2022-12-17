














// 404. Sum of Left Leaves
// Binary Tree 16


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
    pub fn sum_of_left_leaves(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut stack = vec![root];
        let mut sum = 0;
        while !stack.is_empty() {
            let temp = stack.pop().unwrap();
            if temp.is_none() {continue;}
            let node = temp.unwrap();
            let inner = node.borrow();
            stack.push(inner.left.clone());
            stack.push(inner.right.clone());
            if let Some(x) = inner.left.clone() {
                let inner = x.borrow();
                if inner.left.is_none() && inner.right.is_none() {
                    sum += inner.val;
                }
            }
        }
        sum
    }
}

fn main() {
}
