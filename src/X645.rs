// LeetCode, 645. Set Mismatch

fn main() {
    assert_eq!(find_error_nums(vec![1, 1]), vec![1, 2]);
    assert_eq!(find_error_nums(vec![1, 2, 2, 4]), vec![2, 3]);
}

fn find_error_nums(nums: Vec<i32>) -> Vec<i32> {
    let number: usize = nums
        .clone()
        .iter()
        .enumerate()
        .take_while(|(i, j)| *i != **j as usize)
        .count()
        .into();
    vec![nums[number] as i32, number as i32 + 1]
}
