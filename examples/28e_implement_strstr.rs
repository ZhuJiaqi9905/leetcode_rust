impl Solution {
    /// Direct solution.
    pub fn str_str(haystack: String, needle: String) -> i32 {
        if needle.len() == 0 {
            return 0;
        }
        if haystack.len() < needle.len() {
            return -1;
        }
        let haystack = haystack.as_bytes();
        let needle = needle.as_bytes();
        for i in 0..=(haystack.len() - needle.len()) {
            if haystack[i] == needle[0] {
                let mut is_equal = true;
                for j in 1..needle.len() {
                    if haystack[i + j] != needle[j] {
                        is_equal = false;
                        break;
                    }
                }
                if is_equal {
                    return i as i32;
                }
            }
        }
        return -1;
    }
    /// KMP
    pub fn str_str_kmp(haystack: String, needle: String) -> i32 {
        let haystack = haystack.as_bytes();
        let needle = needle.as_bytes();
        let n = haystack.len();
        let m = needle.len();
        if m == 0 {
            return 0;
        }
        if n < m {
            return -1;
        }
        let mut pi = vec![0; m]; // 子串 s[0:i] 的最长的相等的真前缀与真后缀的长度.
        let mut j = 0;
        // Compute the pi[i]
        for i in 1..m {
            while j > 0 && needle[i] != needle[j] {
                j = pi[j - 1];
            }
            if needle[i] == needle[j] {
                j += 1;
            }
            pi[i] = j;
        }
        j = 0;
        for i in 0..n {
            while j > 0 && haystack[i] != needle[j] {
                j = pi[j - 1];
            }
            if haystack[i] == needle[j] {
                j += 1;
            }
            if j == m {
                return (1 + i - m) as i32;
            }
        }
        return -1;
    }
}

struct Solution;
fn main() {
    assert_eq!(Solution::str_str("hello".into(), "ll".into()), 2);
    assert_eq!(Solution::str_str("aaaaa".into(), "bba".into()), -1);
    assert_eq!(Solution::str_str("aaaa".into(), "aaa".into()), 0);
    assert_eq!(Solution::str_str_kmp("hello".into(), "ll".into()), 2);
    assert_eq!(Solution::str_str_kmp("aaaaa".into(), "bba".into()), -1);
    assert_eq!(Solution::str_str_kmp("aaaa".into(), "aaa".into()), 0); 
    assert_eq!(Solution::str_str_kmp("mississippi".into(), "issip".into()), 4); 
}
