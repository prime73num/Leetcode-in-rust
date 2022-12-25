
// 77. Combinations
// BackTrace 2

struct Solution {}
impl Solution {
    fn backtrace(pos: &mut Vec<i32>, res: &mut Vec<Vec<i32>>, n: i32, k: i32) {
        if k == 0 {
            res.push(pos.clone());
            return;
        }
        for i in k..=n {
            pos.push(i);
            Self::backtrace(pos, res, i-1, k-1);
            pos.pop();
        }
    }
    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        let mut res = Vec::new();
        let mut pos = Vec::new();
        Self::backtrace(&mut pos, &mut res, n, k);
        res
    }
}
fn main() {
}
