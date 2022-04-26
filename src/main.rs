struct Solution;
impl Solution {
    pub fn projection_area(grid: Vec<Vec<i32>>) -> i32 {
        let mut xy_size = 0;
        let mut yz_size = 0;
        grid.iter().for_each(|v| {
            let mut max_x = 0;
            v.iter().for_each(|x| {
                if *x != 0 {
                    xy_size += 1;
                }
                max_x = i32::max(max_x, *x);
            });
            yz_size += max_x;
        });
        let n = grid.len();
        let mut xz_size = 0;
        for j in 0..n {
            let mut max_y = 0;
            for i in 0..n {
                max_y = i32::max(max_y, grid[i][j]);
            }
            xz_size += max_y;
        }
        xy_size + yz_size + xz_size
    }
}

fn main() {
    assert_eq!(Solution::projection_area(vec![vec![1, 2], vec![3, 4]]), 17);
    assert_eq!(Solution::projection_area(vec![vec![2]]), 5);
    assert_eq!(Solution::projection_area(vec![vec![1, 0], vec![0, 2]]), 8);
}
