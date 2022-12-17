









// 100. Same Tree

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
    pub fn is_same_tree(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let mut stack_one = vec![p];
        let mut stack_two = vec![q];
        while !stack_one.is_empty() {
            let temp_one = stack_one.pop().unwrap();
            let temp_two = stack_two.pop().unwrap();
            if temp_one.is_none() && temp_two.is_none() { continue; }
            if !(temp_one.is_some() && temp_two.is_some()) {
                return false;
            }

            let node_one = temp_one.unwrap();
            let node_two = temp_two.unwrap();
            let inner_one = node_one.borrow();
            let inner_two = node_two.borrow();
            if inner_one.val != inner_two.val {
                return false;
            }
            stack_one.push(inner_one.right.clone());
            stack_one.push(inner_one.left.clone());
            stack_two.push(inner_two.right.clone());
            stack_two.push(inner_two.left.clone());
        }
        return true;
    }
}

fn main() {
}
