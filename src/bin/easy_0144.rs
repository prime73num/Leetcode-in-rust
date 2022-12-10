










// Binary Tree Preorder Traversal
// Binary Tree 3


use std::rc::Rc;
use std::cell::RefCell;

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


struct Solution {}
impl Solution {
    fn travel(root: Option<&Rc<RefCell<TreeNode>>>, res: &mut Vec<i32>) {
        if root.is_none() { return; }
        let node = root.unwrap();
        let inner = node.borrow();
        res.push(inner.val);
        Self::travel(inner.left.as_ref(), res);
        Self::travel(inner.right.as_ref(), res);
    }
    pub fn preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut res = Vec::new();
        Self::travel(root.as_ref(), &mut res);
        res
    }
    pub fn preorder_traversal_use_stack(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut res:Vec<i32> = Vec::new();
        let mut stack = vec![root];
        while !stack.is_empty() {
            let temp = stack.pop().unwrap();
            if temp.is_none() { continue; }
            let node = temp.unwrap();
            let inner = node.borrow();
            res.push(inner.val);
            stack.push(inner.right.clone());
            stack.push(inner.left.clone());
        }
        res
    }
}

fn main() {
    println!("hello");
}
