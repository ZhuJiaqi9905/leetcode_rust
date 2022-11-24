/// 对于正数，按照从小到大的顺序排，注意如果有0的话，需要10000123这样。
/// 对于负数，从大往小顺序排就行。
/// 对于0，直接返回。
impl Solution {
    pub fn smallest_number(mut num: i64) -> i64 {
        if num == 0 {
            return num;
        }
        let is_pos = num >= 0;
        let mut num_cnt = vec![0; 10];
        num = num.abs();
        while num != 0 {
            let idx = num % 10;
            num_cnt[idx as usize] += 1;
            num /= 10;
        }
        if is_pos {
            let mut i = 1;
            while num_cnt[i] == 0 {
                i += 1;
            }
            let mut ans = 0;
            ans += i;
            num_cnt[i] -= 1;
            for _ in 0..num_cnt[0] {
                ans *= 10;
            }
            while i < 10 {
                for _ in 0..num_cnt[i] {
                    ans *= 10;
                    ans += i;
                }
                i += 1;
            }
            return ans as i64;
        } else {
            let mut ans = 0;
            for i in (0..=9).rev() {
                for _ in 0..num_cnt[i] {
                    ans *= 10;
                    ans += i;
                }
            }
            return -(ans as i64);
        }
    }
}

struct Solution;
fn main() {
    assert_eq!(Solution::smallest_number(310), 103);
    assert_eq!(Solution::smallest_number(-7605), -7650);
}
