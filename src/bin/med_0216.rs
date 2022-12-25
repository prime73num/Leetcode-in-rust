









// 216. Combination Sum III
// BackTrace 4
struct Solution {}
impl Solution {
    fn backtrace(sum: &mut i32, pos: &mut Vec<i32>, res: &mut Vec<Vec<i32>>, n: i32, k: i32) {
        if k == 0 {
            if *sum == 0 {
                res.push(pos.clone());
            }
            return;
        }
        for i in k..=n {
            pos.push(i);
            *sum -= i;
            Self::backtrace(sum, pos, res, i-1, k-1);
            *sum += i;
            pos.pop();
        }
    }
    pub fn combination_sum3(k: i32, n: i32) -> Vec<Vec<i32>> {
        let mut res = Vec::new();
        let mut pos = Vec::new();
        let mut sum = n;
        Self::backtrace(&mut sum, &mut pos, &mut res, 9, k);
        res
    }
}
fn main() {
}
