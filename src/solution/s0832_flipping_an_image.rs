/**
 * [832] Flipping an Image
 *
 * Given a binary matrix A, we want to flip the image horizontally, then invert it, and return the resulting image.
 *
 * To flip an image horizontally means that each row of the image is reversed.  For example, flipping [1, 1, 0] horizontally results in [0, 1, 1].
 *
 * To invert an image means that each 0 is replaced by 1, and each 1 is replaced by 0. For example, inverting [0, 1, 1] results in [1, 0, 0].
 *
 * Example 1:
 *
 *
 * Input: [[1,1,0],[1,0,1],[0,0,0]]
 * Output: [[1,0,0],[0,1,0],[1,1,1]]
 * Explanation: First reverse each row: [[0,1,1],[1,0,1],[0,0,0]].
 * Then, invert the image: [[1,0,0],[0,1,0],[1,1,1]]
 *
 *
 * Example 2:
 *
 *
 * Input: [[1,1,0,0],[1,0,0,1],[0,1,1,1],[1,0,1,0]]
 * Output: [[1,1,0,0],[0,1,1,0],[0,0,0,1],[1,0,1,0]]
 * Explanation: First reverse each row: [[0,0,1,1],[1,0,0,1],[1,1,1,0],[0,1,0,1]].
 * Then invert the image: [[1,1,0,0],[0,1,1,0],[0,0,0,1],[1,0,1,0]]
 *
 *
 * Notes:
 *
 *
 * 	1 <= A.length = A[0].length <= 20
 * 	0 <= A[i][j]<font face="sans-serif, Arial, Verdana, Trebuchet MS"> <= </font>1
 *
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/flipping-an-image/
// discuss: https://leetcode.com/problems/flipping-an-image/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn flip_and_invert_image(a: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut result = a;
        let m = result.first().map_or(0, |row| row.len()) as usize;
        for row in &mut result {
            let (mut l, mut r) = (0, m - 1);
            while l < r {
                if row[l] == row[r] {
                    row[l] = row[l] ^ 1;
                    row[r] ^= 1;
                }
                l += 1;
                r -= 1;
            }
            if l == r {
                row[l] ^= 1;
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
    fn test_832() {
        let input = vec![vec![1, 1, 0], vec![1, 0, 1], vec![0, 0, 0]];
        let expect = vec![vec![1, 0, 0], vec![0, 1, 0], vec![1, 1, 1]];
        assert_eq!(Solution::flip_and_invert_image(input), expect)
    }
}
