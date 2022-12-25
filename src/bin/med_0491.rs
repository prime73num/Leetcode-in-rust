












// 491. Non-decreasing Subsequences
// BackTrace 14
use std::collections::hash_set::HashSet;
struct Solution {}
impl Solution {
    fn backtrace(pos: &mut Vec<i32>, res: &mut Vec<Vec<i32>>, nums: &[i32], abort: &mut HashSet<i32>) {
        if nums.is_empty() {
            if pos.len() >= 2 {
                res.push(pos.clone());
            }
            return;
        }
        let head = nums[0];
        if pos.is_empty() || *pos.last().unwrap() <= head {
            if !abort.contains(&head) {
                pos.push(head);
                let temp = std::mem::replace(abort, HashSet::new());
                Self::backtrace(pos, res, &nums[1..], abort);
                pos.pop();
                let _ = std::mem::replace(abort, temp);
            }
        }
        let has = abort.contains(&head);
        if !has {
            abort.insert(head);
        }
        Self::backtrace(pos, res, &nums[1..], abort);
        if !has {
            abort.remove(&head);
        }
    }
    pub fn find_subsequences(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut pos = Vec::new();
        let mut res = Vec::new();
        let mut abort = HashSet::new();
        Self::backtrace(&mut pos, &mut res, &nums, &mut abort);
        res
    }
}
fn main() {
}
