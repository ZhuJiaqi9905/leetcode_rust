/// 使用并查集，要维护最小的元素是根元素。
impl Solution {
    pub fn smallest_equivalent_string(s1: String, s2: String, base_str: String) -> String {
        let s1 = s1.as_bytes();
        let s2 = s2.as_bytes();
        let len = s1.len();
         const base: u8 = 'a' as u8;
        let mut union = Union::new();
        for i in 0..len {
            union.union((s1[i] - base) as i32, (s2[i] - base) as i32);
        }
        let mut res = String::new();
        let base_str = base_str.as_bytes();
        let len = base_str.len();
        for i in 0..len {
            let c = char::from_u32((base as i32 + union.find_parent((base_str[i] - base) as i32)) as u32).unwrap();
            res.push(c);
        }
        return res;
    }
}
struct Union {
    nums: Vec<i32>,
}
impl Union {
    pub fn new() -> Self {
        let nums = vec![-1; 26];
        Self { nums }
    }
    pub fn find_parent(&mut self, x: i32) -> i32 {
        if self.nums[x as usize] < 0 {
            return x;
        } else {
            self.nums[x as usize] = self.find_parent(self.nums[x as usize]);
            return self.nums[x as usize];
        }
    }
    pub fn union(&mut self, x: i32, y: i32) {
        let par_x = self.find_parent(x);
        let par_y = self.find_parent(y);
        if par_x == par_y {
            return;
        }
        // 因为题目中要找到每个集合中字典序最小的元素。
        // 因此把字典序大的根元素的集合，往字典序小的根元素的集合中合并
        if par_x > par_y {
            self.nums[par_x as usize] = par_y;
        } else {
            self.nums[par_y as usize] = par_x;
        }
    }
}
struct Solution;
fn main() {
    assert_eq!(
        Solution::smallest_equivalent_string("parker".into(), "morris".into(), "parser".into()),
        "makkek".to_string()
    );
    assert_eq!(
        Solution::smallest_equivalent_string("hello".into(), "world".into(), "hold".into()),
        "hdld".to_string()
    );
    assert_eq!(
        Solution::smallest_equivalent_string(
            "leetcode".into(),
            "programs".into(),
            "sourcecode".into()
        ),
        "aauaaaaada".to_string()
    );
}
