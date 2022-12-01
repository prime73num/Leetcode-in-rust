










struct Solution {}



impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        let size = matrix.len();
        let mid_i = (size - 1) / 2 + 1;
        let mid_j = (size / 2);
        let mut temp: i32;
        for i in 0..mid_i {
            for j in 0..mid_j {
                temp = matrix[i][j];
                matrix[i][j] = matrix[size - 1 -j][i];
                matrix[size - 1 - j][i] = matrix[size - 1 - i][size - 1 - j];
                matrix[size -1 -i][size - 1 - j] = matrix[j][size - 1 - i];
                matrix[j][size - 1 - i] =  temp;
            }
        }
    }
}



fn main() {
}
