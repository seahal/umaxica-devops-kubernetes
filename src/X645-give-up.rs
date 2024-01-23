// LeetCode, 645. Set Mismatch

fn main() {
    assert_eq!(find_error_nums(vec![1, 2, 2, 4]), vec![2, 3]);
    assert_eq!(find_error_nums(vec![1, 1]), vec![1, 2]);
    assert_eq!(find_error_nums(vec![2, 2]), vec![2, 1]);
    assert_eq!(find_error_nums(vec![3, 2, 2]), vec![2, 1]);
}

fn find_error_nums(nums: Vec<i32>) -> Vec<i32> {
    let number = nums
        .windows(2)
        .zip(1..)
        .take_while(|(fst, snd)| (**fst)[0] == *snd)
        .count();

    vec![nums[number] as i32, number as i32 + 1]
}
