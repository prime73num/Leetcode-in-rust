// 406. Queue Reconstruction by Height
// Greedy 14
struct Solution {}
impl Solution {
    pub fn reconstruct_queue(mut people: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        people.sort_by(|x,y| {
            let res = y[0].cmp(&x[0]);
            if res==std::cmp::Ordering::Equal {
                return x[1].cmp(&y[1]);
            }
            return res;
        });
        let mut res = Vec::new();
        for i in people {
            let idx = i[1] as usize;
            res.insert(idx, i);
        }
        res
    }
}
fn main() {
}
