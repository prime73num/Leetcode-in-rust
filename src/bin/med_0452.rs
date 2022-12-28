// 452. Minimum Number of Arrows to Burst Balloons
// Greedy 17
struct Solution {}
impl Solution {
    pub fn find_min_arrow_shots(mut points: Vec<Vec<i32>>) -> i32 {
        points.sort();
        let mut big_end = std::i64::MIN;
        let mut sum = 0;
        for i in &points {
            let start = i[0];
            let end = i[1] as i64;
            if start as i64 > big_end {
                sum += 1;
                big_end = end;
            } else {
                if end < big_end {
                    big_end = end;
                }
            }
        }
        sum
    }
}
fn main() {
}
