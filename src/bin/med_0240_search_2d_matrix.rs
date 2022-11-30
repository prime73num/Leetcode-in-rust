







struct Solution {}

impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let (row, col) = (matrix.len(), matrix[0].len());
        let mut row_p = 0;
        let mut col_p = (col -1) as isize;
        while row_p < row && col_p >= 0 {
            let temp = matrix[row_p as usize][col_p as usize];
            if temp == target {
                return true;
            } else if temp > target {
                col_p -= 1;
            } else {
                row_p += 1;
            }
        }
        false
    }
}




fn main() {
}
