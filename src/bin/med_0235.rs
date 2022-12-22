




// 235. Lowest Common Ancestor of a Binary Search Tree
// Binary Tree 29
//

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
    pub fn lowest_common_ancestor(root: Option<Rc<RefCell<TreeNode>>>, p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        let nodep = p.unwrap();
        let nodeq = q.unwrap();
        let valp = nodep.borrow().val;
        let valq = nodeq.borrow().val;
        let mut res = root;
        while res.is_some() {
            let node = res.as_ref().unwrap();
            let temp = node.borrow().val;
            if valp < temp && valq < temp {
                res = res.unwrap().borrow().left.clone();
            } else if valp > temp && valq > temp {
                res = res.unwrap().borrow().right.clone();
            } else {
                return res;
            }
        }
        None
    }
}
fn main() {
}
