// 338. Counting Bits

fn main() {
    assert_eq!(vec![0, 1, 1], count_bits(2));
    assert_eq!(vec![0, 1, 1, 2, 1, 2], count_bits(5));
}

//impl Solution {
pub fn count_bits(n: i32) -> Vec<i32> {
    (0..=n)
        .map(|m| {
            format!("{:0b}", m)
                .matches(char::is_numeric)
                .filter(|c| *c == "1")
                .count() as i32
        })
        .collect::<Vec<_>>()
}
//}
