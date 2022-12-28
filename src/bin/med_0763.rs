// 763. Partition Labels
// Greedy 19
struct Solution {}
impl Solution {
    pub fn partition_labels(s: String) -> Vec<i32> {
        let mut last = [0;26];
        let bytes = s.as_bytes();
        for i in 0..s.as_bytes().len() {
            let idx = bytes[i] - b'a';
            last[idx as usize] = i;
        }
        let mut end = -1;
        let mut res = Vec::new();
        for i in 0..bytes.len() {
            let idx = (bytes[i] - b'a') as usize;
            if i as i32 > end {
                res.push(i as i32);
            }
            if last[idx] as i32 > end {
                end = last[idx] as i32;
            }
        }
        res.push(bytes.len() as i32);
        for i in 0..res.len()-1 {
            res[i] = res[i+1] - res[i];
        }
        res.pop();
        res
    }
}
fn main() {
}
