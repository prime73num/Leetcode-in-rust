// 56. Merge Intervals
// Greedy 20
struct Solution {}
impl Solution {
    pub fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        intervals.sort();
        let mut start = intervals[0][0];
        let mut end = start;
        let mut res = Vec::new();
        for i in &intervals {
            let s = i[0];
            let e = i[1];
            if s > end {
                res.push(vec![start,end]);
                start = s;
            }
            if e > end {
                end = e;
            }
        }
        res.push(vec![start,end]);
        res
    }
}
fn main() {
}
