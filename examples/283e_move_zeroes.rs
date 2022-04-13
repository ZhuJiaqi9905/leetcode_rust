impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut i = 0;
        for j in 0..nums.len() {
            if nums[j] != 0 {
                nums[i] = nums[j];
                i += 1;
            }
        }
        while i < nums.len() {
            nums[i] = 0;
            i += 1;
        }
    }
}

struct Solution;

fn main() {
    let mut nums: Vec<i32> = vec![0, 1, 0, 3, 9];
    Solution::move_zeroes(&mut nums);
    println!("{:?}", nums);
}
