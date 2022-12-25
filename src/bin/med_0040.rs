// 40. Combination Sum II
// BackTrace 8
struct Solution {}
impl Solution {
    fn backtrace(res: &mut Vec<Vec<i32>>, pos: &mut Vec<i32>, target: &mut i32, count: &mut [i32], candidates: &[i32]) {
        if *target < 0 { return; }
        if *target == 0 {
            res.push(pos.clone());
        }
        for i in 0..candidates.len() {
            pos.push(candidates[i]);
            *target -= candidates[i];
            count[i] -= 1;
            if count[i] == 0 {
                Self::backtrace(res, pos, target, count, &candidates[..i]);
            } else {
                Self::backtrace(res, pos, target, count, &candidates[..=i]);
            }
            count[i] += 1;
            *target += candidates[i];
            pos.pop();
        }
    }
    pub fn combination_sum2(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut map = std::collections::hash_map::HashMap::new();
        for i in candidates {
            *map.entry(i).or_insert(0) += 1;
        }
        let mut numx = Vec::new();
        let mut count = Vec::new();
        for i in map.iter() {
            numx.push(*i.0);
            count.push(*i.1);
        }
        let mut res = Vec::new();
        let mut pos = Vec::new();
        let mut target = target;
        Self::backtrace(&mut res, &mut pos, &mut target, &mut count, &numx);
        res
    }
}
fn main() {
}
