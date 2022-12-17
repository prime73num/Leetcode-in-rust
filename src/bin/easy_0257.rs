














// 257. Binary Tree Paths
// Binary Tree 13

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
    fn travel(root: Option<Rc<RefCell<TreeNode>>>, path: &mut String, res: &mut Vec<String>) {
        let node = root.unwrap();
        let inner = node.borrow();
        let val = inner.val;
        let len = path.len();
        path.push_str(&val.to_string());
        if inner.left.is_none() && inner.right.is_none() {
            res.push(path.clone());
            path.truncate(len);
            return;
        }
        path.push_str(&"->");
        if inner.left.is_some() {
            Self::travel(inner.left.clone(), path, res);
        }
        if inner.right.is_some() {
            Self::travel(inner.right.clone(), path, res);
        }
        path.truncate(len);
    }
    pub fn binary_tree_paths(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<String> {
        let mut path = String::new();
        let mut res = Vec::new();
        if root.is_none() {
            return res;
        }
        Self::travel(root, &mut path, &mut res);
        res
    }
}
fn main() {
}

