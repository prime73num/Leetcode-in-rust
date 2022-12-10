

// 145. Binary Tree Postorder Traversal
// Binary Tree 3


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
        Self::travel(inner.right.as_ref(), res);
        res.push(inner.val);
    }
    pub fn postorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut res = Vec::new();
        Self::travel(root.as_ref(), &mut res);
        res
    }
    pub fn postorder_traversal_with_stack(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut res:Vec<i32> = Vec::new();
        let mut stack = vec![root];
        while !stack.is_empty() {
            let temp = stack.pop().unwrap();
            if temp.is_none() { continue; }
            let node = temp.unwrap();
            let inner = node.borrow();
            if !stack.is_empty() && stack.last().unwrap().is_none() {
                stack.pop();
                res.push(inner.val);
                continue;
            }
            let left = inner.left.clone();
            let right = inner.right.clone();
            drop(inner);
            stack.push(None);
            stack.push(Some(node));
            if right.is_some() {
                stack.push(right);
            }
            if left.is_some() {
                stack.push(left);
            }
        }
        res
    }
}

fn main() {
}
