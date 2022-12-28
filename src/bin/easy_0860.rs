

















// 860. Lemonade Change
// Greedy 13
struct Solution {}
impl Solution {
    pub fn lemonade_change(bills: Vec<i32>) -> bool {
        let mut fives = 0;
        let mut tens = 0;
        for i in bills {
            if i==5 {
                fives += 1;
            } else if i==10 {
                fives -= 1;
                tens += 1;
            } else if i==20 {
                if tens >= 1 {
                    tens -= 1;
                    fives -= 1;
                } else {
                    fives -= 3;
                }
            }
            if fives < 0 || tens < 0 {
                return false;
            }
        }
        return true;
    }
}
fn main() {
}
