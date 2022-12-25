










// 39. Combination Sum
// BackTrace 7
struct Solution {}
impl Solution {
    fn backtrace(res: &mut Vec<Vec<i32>>, pos: &mut Vec<i32>, target: &mut i32, candidates: &[i32]) {
        if *target < 0 { return; }
        if *target == 0 {
            res.push(pos.clone());
        }
        for i in 0..candidates.len() {
            pos.push(candidates[i]);
            *target -= candidates[i];
            Self::backtrace(res, pos, target, &candidates[..=i]);
            *target += candidates[i];
            pos.pop();
        }
    }
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut target = target;
        let mut res = Vec::new();
        let mut pos = Vec::new();
        Self::backtrace(&mut res, &mut pos, &mut target, &candidates);
        res
    }
}
fn main() {
}
