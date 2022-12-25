













// 46. Permutations
// BackTrace 15
struct Solution {}
impl Solution {
    fn backtrace(pos: &mut Vec<i32>, res: &mut Vec<Vec<i32>>, nums: &[i32], valid: &mut [bool]) {
        if nums.is_empty() {
            res.push(pos.clone());
            return;
        }
        let head = nums[0];
        for i in 0..pos.len() {
            if valid[i] {
                pos[i] = head;
                valid[i] = false;
                Self::backtrace(pos, res, &nums[1..], valid);
                valid[i] = true;
            }
        }
    }
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut pos = vec![0;nums.len()];
        let mut res = Vec::new();
        let mut valid = vec![true;nums.len()];
        Self::backtrace(&mut pos, &mut res, &nums, &mut valid);
        res
    }
}
fn main() {
}
