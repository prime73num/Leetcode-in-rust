













// 78. Subsets
// BackTrace 11
struct Solution {}
impl Solution {
    fn backtrace(pos: &mut Vec<i32>,res: &mut Vec<Vec<i32>>, nums: &[i32]) {
        if nums.is_empty() {
            res.push(pos.clone());
            return;
        }
        let first = nums[0];
        Self::backtrace(pos, res, &nums[1..]);
        pos.push(first);
        Self::backtrace(pos, res, &nums[1..]);
        pos.pop();
    }
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut pos = Vec::new();
        let mut res = Vec::new();
        Self::backtrace(&mut pos, &mut res, &nums);
        res
    }
}
fn main() {
}
