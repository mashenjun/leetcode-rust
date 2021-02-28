/**
 * [867] Transpose Matrix
 *
 * Given a 2D integer array matrix, return the transpose of matrix.
 * The transpose of a matrix is the matrix flipped over its main diagonal, switching the matrix's row and column indices.
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/02/10/hint_transpose.png" style="width: 600px; height: 197px;" />
 *  
 * Example 1:
 *
 * Input: matrix = [[1,2,3],[4,5,6],[7,8,9]]
 * Output: [[1,4,7],[2,5,8],[3,6,9]]
 *
 * Example 2:
 *
 * Input: matrix = [[1,2,3],[4,5,6]]
 * Output: [[1,4],[2,5],[3,6]]
 *
 *  
 * Constraints:
 *
 * 	m == matrix.length
 * 	n == matrix[i].length
 * 	1 <= m, n <= 1000
 * 	1 <= m * n <= 10^5
 * 	-10^9 <= matrix[i][j] <= 10^9
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/transpose-matrix/
// discuss: https://leetcode.com/problems/transpose-matrix/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn transpose(matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let (n, m) = (
            matrix.len(),
            matrix.first().map_or(0, |row| row.len()) as usize,
        );
        let mut result = vec![vec![0i32; n]; m];
        for (i, row) in matrix.iter().enumerate() {
            for (j, val) in row.iter().enumerate() {
                result[j][i] = *val;
            }
        }
        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_867() {
        let matrix = vec![vec![1, 2, 3], vec![4, 5, 6]];
        let expect = vec![vec![1, 4], vec![2, 5], vec![3, 6]];
        assert_eq!(Solution::transpose(matrix), expect);
    }
}
