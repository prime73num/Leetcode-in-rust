









// 106. Construct Binary Tree from Inorder and Postorder Traversa
// Binary Tree 19
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
    fn travel(inorder: &[i32], postorder: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
        if postorder.is_empty() {
            return None;
        }
        let val = *postorder.last().unwrap();
        let mut root = TreeNode::new(val);
        let idx = inorder.iter().enumerate().find(|(_,y)| {
            return **y==val;
        }).unwrap().0;
        let left = Self::travel(&inorder[..idx], &postorder[..idx]);
        let right = Self::travel(&inorder[idx+1..], &postorder[idx..postorder.len()-1]);
        root.left = left;
        root.right = right;
        Some(Rc::new(RefCell::new(root)))
    }
    pub fn build_tree(inorder: Vec<i32>, postorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        return Self::travel(&inorder, &postorder);
    }
}
fn main() {
}
