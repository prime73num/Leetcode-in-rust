










// 501. Find Mode in Binary Search Tree
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
    fn travel(root: Option<Rc<RefCell<TreeNode>>>, val: &mut i32, count: &mut usize, res: &mut usize, list: &mut Vec<i32>) {
        if root.is_none() {
            return ;
        }
        let node = root.unwrap();
        let inner = node.borrow();
        Self::travel(inner.left.clone(), val, count, res, list);
        if *val != inner.val {
            if *res < *count {
                *res = *count;
                list.clear();
                list.push(*val);
            } else if *res == *count {
                list.push(*val);
            }
            *count = 0;
        }
        *count += 1;
        *val = inner.val;
        Self::travel(inner.right.clone(), val, count, res, list);
        return;
    }
    pub fn find_mode(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut val = std::i32::MIN;
        let mut count = 0;
        let mut res = 0;
        let mut list = Vec::new();
        Self::travel(root, &mut val, &mut count, &mut res, &mut list);
        if res < count {
            list.clear();
            list.push(val);
        } else if res == count {
            list.push(val);
        }
        return list;
    }
}
fn main() {
}
