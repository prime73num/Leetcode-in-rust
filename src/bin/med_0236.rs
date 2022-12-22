






// 236. Lowest Common Ancestor of a Binary Tree
// Binary Tree 26


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
    fn travel(root: Option<Rc<RefCell<TreeNode>>>, p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>, res: &mut Option<Rc<RefCell<TreeNode>>>) -> usize {
        if root .is_none() {
            *res = None;
            return 0;
        }
        let node = root.unwrap();
        let inner = node.borrow();
        let left = Self::travel(inner.left.clone(), p.clone(), q.clone(), res);
        if left == 3 {
            return 3;
        }
        let right = Self::travel(inner.right.clone(), p.clone(), q.clone(), res);
        if right == 3 {
            return 3;
        }
        drop(inner);
        let mut mid = 0;
        if node == p.unwrap() {
            mid = 1;
        }
        if node == q.unwrap() {
            mid = 2;
        }
        let sum = left + right + mid;
        if sum==3 {
            *res = Some(node.clone());
            return 3;
        }
        return sum;
    }
    pub fn lowest_common_ancestor(root: Option<Rc<RefCell<TreeNode>>>, p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut res = None;
        Self::travel(root, p, q, &mut res);
        return res;
    }
}
fn main() {
}
