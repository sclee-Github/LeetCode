pub fn get_how_many_number(nums: Vec<i32>) -> Vec<i32> {
    // println!("{:?}", my_vec.iter().map(|x| my_vec.iter().filter(|y| *y < x).count() as i32).collect::<Vec<i32>>());

    nums.iter()
        .map(|x| nums.iter().filter(|y| *y < x).count() as i32)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn how_many_numbers_are_smaller_than_current_number() {
        let my_vec: Vec<i32> = vec![8, 1, 2, 2, 3];
        let test_vec: Vec<i32> = vec![4, 0, 1, 1, 3];

        assert_eq!(get_how_many_number(my_vec), test_vec);
    }
}
