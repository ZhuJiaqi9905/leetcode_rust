impl Solution {
    pub fn fib(n: i32) -> i32 {
        // 保证至少有2个元素
        let mut ans = vec![-1; (n + 2) as usize]; 
        ans[0] = 0;
        ans[1] = 1;
        return Solution::dfs_fib(n as usize, &mut ans);
    }
    fn dfs_fib(n: usize, ans: &mut Vec<i32>) -> i32 {
        if ans[n] != -1 {
            return ans[n];
        }
        ans[n] = Solution::dfs_fib(n - 1, ans) + Solution::dfs_fib(n - 2, ans);
        return ans[n];
    }
}
struct Solution;
fn main() {
    assert_eq!(0, Solution::fib(0));
    assert_eq!(1, Solution::fib(2));
    assert_eq!(2, Solution::fib(3));
    assert_eq!(3, Solution::fib(4));
}
