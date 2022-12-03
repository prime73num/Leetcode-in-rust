













// Spiral Matrix II
// Array 6



struct Solution {}

impl Solution {
    pub fn generate_matrix(n: i32) -> Vec<Vec<i32>> {
        let direction = [
            [0, 1], [1, 0], [0, -1], [-1, 0]
        ];
        let (mut row_s, mut row_e, mut col_s, mut col_e) = (0, n, 0, n);
        let size = row_e * col_e;

        let mut res = vec![vec![0;n as usize];n as usize];
        let mut p = 0;
        let (mut x, mut y) = (0 as i32, 0 as i32);
        for i in 1..=size {
            res[x as usize][y as usize] = i;

            let temp_x = x + direction[p][0];
            let temp_y = y + direction[p][1];
            if !(temp_x >= row_s && temp_x < row_e && temp_y >= col_s && temp_y < col_e) {
                if p == 0 {
                    row_s += 1;
                } else if p==1 {
                    col_e -= 1;
                } else if p==2 {
                    row_e -= 1;
                } else if p==3 {
                    col_s += 1;
                }
                p += 1;
                p %= 4;
            }
            x = x + direction[p][0];
            y = y + direction[p][1];
        }
        res
    }
}

fn main() {
}
