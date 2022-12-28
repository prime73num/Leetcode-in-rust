// 435. Non-overlapping Intervals
// Greedy 18
struct Solution {}
impl Solution {
    pub fn erase_overlap_intervals(mut intervals: Vec<Vec<i32>>) -> i32 {
        intervals.sort();
        let mut big_end = std::i64::MIN;
        let mut sum = 0;
        for i in &intervals {
            let start = i[0];
            let end = i[1] as i64;
            if start as i64 >= big_end {
                sum += 1;
                big_end = end;
            } else {
                if end < big_end {
                    big_end = end;
                }
            }
        }
        (intervals.len()-sum) as i32
    }
}
fn main() {
}
