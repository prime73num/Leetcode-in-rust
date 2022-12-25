// 51. N-Queens
// BackTrace 20
struct Solution {}
impl Solution {
    fn valid(pos: &Vec<i32>) -> bool {
        let idx_last = pos.len() - 1;
        let last = pos[idx_last];
        for i in 0..idx_last {
            let temp = pos[i];
            if temp == last || last - temp == (idx_last - i) as i32 || last - temp == (i-idx_last) as i32{
                return false;
            }
        }
        return true;
    }
    fn backtrace(pos: &mut Vec<i32>, res:&mut Vec<Vec<i32>>, n: i32) {
        if pos.len() == n as usize {
            res.push(pos.clone());
            return;
        }
        for i in 0..n {
            pos.push(i);
            if Self::valid(pos) {
                Self::backtrace(pos, res, n);
            }
            pos.pop();
        }
    }
    pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
        let mut pos = Vec::new();
        let mut res = Vec::new();
        Self::backtrace(&mut pos, &mut res, n);
        let mut str_res = Vec::new();
        for i in res {
            let mut temp = Vec::new();
            for num in i {
                let mut row = vec![b'.';n as usize];
                row[num as usize] = b'Q';
                temp.push(String::from_utf8(row).unwrap())
            }
            str_res.push(temp);
        }
        str_res
    }
}
fn main() {
}
