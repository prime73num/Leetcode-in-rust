// 343. Integer Break
// Dynamic Programming 8
struct Solution {}
impl Solution {
    pub fn integer_break(n: i32) -> i32 {
        if n==3 {return 2;}
        if n==2 {return 1;}
        let count = n/3;
        let m = n%3;
        let mut res = 1;
        for _ in 0..count {
            res *= 3;
        }
        if m == 1 {
            res /= 3;
            res *= 4;
        } else if m==2 {
            res *= 2;
        }
        return res;
    }
}
fn main() {
}
