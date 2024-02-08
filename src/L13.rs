// LeetCode, 13. Roman to Integer

fn main() {
    let mut buffer = String::new();
    let _ = std::io::stdin().read_line(&mut buffer);
    let n = buffer.trim();

    println!("{}", roman_to_int(n.to_owned()));
}

pub fn roman_to_int(mut s: String) -> i32 {
    s = s.replace("IV", "IIII");
    s = s.replace("IX", "VIIII");
    s = s.replace("XL", "XXXX");
    s = s.replace("XC", "LXXXX");
    s = s.replace("CD", "CCCC");
    s = s.replace("CM", "DCCCC");

    let x = s
        .chars()
        .map(|n| match n {
            'I' => 1,
            'V' => 5,
            'X' => 10,
            'L' => 50,
            'C' => 100,
            'D' => 500,
            'M' => 1000,
            _ => 0,
        })
        .sum::<i32>();
    x
}

#[test]
fn test_one() {
    assert_eq!(roman_to_int("I".to_owned()), 1);
    assert_eq!(roman_to_int("IV".to_owned()), 4);
    assert_eq!(roman_to_int("IX".to_owned()), 9);
    assert_eq!(roman_to_int("XL".to_owned()), 40);
    assert_eq!(roman_to_int("XC".to_owned()), 90);
    assert_eq!(roman_to_int("CD".to_owned()), 400);
    assert_eq!(roman_to_int("CM".to_owned()), 900);
    assert_eq!(roman_to_int("MCMXCIV".to_owned()), 1994);
    assert_eq!(roman_to_int("III".to_owned()), 3);
    assert_eq!(roman_to_int("LVIII".to_owned()), 58);
}
