// 968. Binary Tree Cameras
// Greedy 24
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
            right: None,
        }
    }
}
use std::cell::RefCell;
use std::rc::Rc;
struct Solution {}
impl Solution {
    fn is_cover(
        root: Option<Rc<RefCell<TreeNode>>>,
        sum: &mut i32,
        is_covered: &mut bool,
        has_camera: &mut bool,
    ) {
        if root.is_none() {
            *sum = 0;
            *is_covered = true;
            *has_camera = false;
            return;
        }
        let node = root.unwrap();
        let inner = node.borrow();
        let mut res = 0;
        let mut camera = false;
        let mut covered = false;
        Self::is_cover(inner.left.clone(), sum, is_covered, has_camera);
        res += *sum;
        if *has_camera { covered = true; }
        if !*is_covered { camera = true; }
        Self::is_cover(inner.right.clone(), sum, is_covered, has_camera);
        res += *sum;
        if *has_camera { covered = true; }
        if !*is_covered { camera = true; }
        if camera { 
            res += 1;
            covered = true; 
        }
        *sum = res;
        *is_covered = covered;
        *has_camera = camera;
    }
    pub fn min_camera_cover(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut sum = 0;
        let mut is_covered = false;
        let mut has_camera = false;
        Self::is_cover(root, &mut sum, &mut is_covered, &mut has_camera);
        if !is_covered {
            sum += 1;
        }
        return sum;
    }
}
fn main() {}
