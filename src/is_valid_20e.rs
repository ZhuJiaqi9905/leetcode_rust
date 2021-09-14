struct Solution;
impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack: Vec<char> = Vec::new();
        for ch in s.chars() {
            match ch {
                '{' | '(' | '[' => {
                    stack.push(ch);
                },
                '}' => {
                    if stack.is_empty() || stack.pop().unwrap() != '{' {
                        return false;
                    }
                },
                ')' => {
                    if stack.is_empty() || stack.pop().unwrap() != '(' {
                        return false;
                    }
                },
                ']' => {
                    if stack.is_empty() || stack.pop().unwrap() != '[' {
                        return false;
                    }
                },
                _ => (),
            }
        }
        stack.len() == 0
    }
}

fn main() {
    let s =String::from("()[]{}");
    println!("{}", Solution::is_valid(s));
}
