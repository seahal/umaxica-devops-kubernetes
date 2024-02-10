// LeetCode, 20. Valid Parentheses, https://leetcode.com/problems/valid-parentheses/description/
fn main() {}

struct Solution;
impl Solution {
    pub fn is_valid(s: String) -> bool {
        use std::collections::VecDeque;
        let mut vd: VecDeque<char> = VecDeque::new();

        for c in s.chars() {
            match c {
                x @ ('(' | '{' | '[') => vd.push_back(x),
                x @ (')' | '}' | ']') => {
                    let length = vd.len();
                    if length == 0 {
                        return false;
                    }
                    let y = vd[length - 1];
                    match (x, y) {
                        (')', '(') => vd.pop_back(),
                        ('}', '{') => vd.pop_back(),
                        (']', '[') => vd.pop_back(),
                        _ => return false,
                    };
                }
                _ => panic!(),
            }
        }

        vd.len() == 0
    }
}
#[test]
fn test_one() {
    assert_eq!(Solution::is_valid("{}".to_string()), true);
    assert_eq!(Solution::is_valid("{{".to_string()), false);
    assert_eq!(Solution::is_valid("{]".to_string()), false);
    assert_eq!(Solution::is_valid("([)]".to_string()), false);
    assert_eq!(Solution::is_valid("]".to_string()), false);
}
