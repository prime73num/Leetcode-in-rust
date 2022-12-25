// 47. Permutations II
// BackTrace 16


use std::collections::hash_map::HashMap;
struct Solution {}
impl Solution {
    fn backtrace(pos: &mut Vec<i32>, res: &mut Vec<Vec<i32>>, nums: &[i32], valid: &mut [bool], last: &mut HashMap<i32,i32>) {
        if nums.is_empty() {
            res.push(pos.clone());
            return;
        }
        let head = nums[0];
        let last_pos = *last.entry(head).or_insert(-1);
        for i in 0..pos.len() {
            if i as i32 > last_pos && valid[i] {
                pos[i] = head;
                valid[i] = false;
                *last.entry(head).or_insert(-1) = i as i32;
                Self::backtrace(pos, res, &nums[1..], valid, last);
                *last.entry(head).or_insert(-1) = last_pos;
                valid[i] = true;
            }
        }
    }
    pub fn permute_unique(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut pos = vec![0;nums.len()];
        let mut res = Vec::new();
        let mut valid = vec![true;nums.len()];
        let mut last = HashMap::new();
        Self::backtrace(&mut pos, &mut res, &nums, &mut valid, &mut last);
        res
    }
}
fn main() {
}
