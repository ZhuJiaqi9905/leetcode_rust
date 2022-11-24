/// 从左往右遍历，更新max_right的值。max_right代表当前位置时能够到达的最远位置。
impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        if nums.len() <= 1 {
            return true;
        }
        let mut max_right = nums[0];
        let mut pos = 1;
        while pos <= max_right && pos < nums.len() as i32 {
            max_right = max_right.max(nums[pos as usize] + pos);
            pos += 1;
        }
        return nums.len() - 1 <= max_right as usize;
    }
}

struct Solution;
fn main() {
    assert_eq!(Solution::can_jump(vec![2, 3, 1, 1, 4]), true);
    assert_eq!(Solution::can_jump(vec![3, 2, 1, 0, 4]), false);
}
