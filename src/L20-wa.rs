// LeetCode, 20. Valid Parentheses, https://leetcode.com/problems/valid-parentheses/description/

fn main() {}

pub fn is_valid(s: String) -> bool {
    use std::collections::VecDeque;

    let mut vd: VecDeque<char> = VecDeque::new();

    for c in s.chars() {
        match c {
            x @ ('(' | '{' | '[') => vd.push_back(x),
            ')' => {
                let n = vd.binary_search(&'(');
                if let Some(y) = n.ok() {
                    vd.remove(y);
                }
            }
            '}' => {
                let n = vd.binary_search(&'{');
                if let Some(y) = n.ok() {
                    vd.remove(y);
                }
            }
            ']' => {
                let n = vd.binary_search(&'[');
                if let Some(y) = n.ok() {
                    vd.remove(y);
                }
            }
            _ => panic!(),
        }
    }

    vd.len() == 0
}

#[test]
fn test_one() {
    assert_eq!(is_valid("{}".to_string()), true);
    assert_eq!(is_valid("{{".to_string()), false);
    assert_eq!(is_valid("{]".to_string()), false);
    assert_eq!(is_valid("([)]".to_string()), false);
}
