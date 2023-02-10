//https://www.codewars.com/kata/5ac6932b2f317b96980000ca/train/rust
use std::collections::HashSet;
pub fn min_value(mut digits: Vec<i32>) -> i32 {
    let mut hash = digits
        .into_iter()
        .collect::<HashSet<_>>()
        .into_iter()
        .map(|d| d.to_string())
        .collect::<Vec<String>>();
    hash.sort_unstable();
    hash.join("").parse::<i32>().unwrap()
}

#[test]
fn min_value_test() {
    assert_eq!(min_value(vec![1, 3, 1]), 13);
    assert_eq!(min_value(vec![4, 7, 5, 7]), 457);
    assert_eq!(min_value(vec![4, 8, 1, 4]), 148);
}
