impl Solution {
    pub fn projection_area(grid: Vec<Vec<i32>>) -> i32 {
        let mut xy_size = 0;
        let mut yz_size = 0;
        let n = grid.len();
        let mut xz_size = 0;
        for i in 0..n {
            let mut max_y = 0;
            let mut max_x = 0;
            for j in 0..n {
                max_y = i32::max(max_y, grid[j][i]);
                max_x = i32::max(max_x, grid[i][j]);
                if grid[i][j] != 0 {
                    xy_size += 1;
                }
            }
            xz_size += max_y;
            yz_size += max_x;
        }

        xy_size + yz_size + xz_size
    }
}


struct Solution;

fn main() {
    assert_eq!(Solution::projection_area(vec![vec![1, 2], vec![3, 4]]), 17);
    assert_eq!(Solution::projection_area(vec![vec![2]]), 5);
    assert_eq!(Solution::projection_area(vec![vec![1, 0], vec![0, 2]]), 8);
}
