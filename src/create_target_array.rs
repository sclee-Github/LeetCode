pub fn get_create_target_array(nums: Vec<i32>, index: Vec<i32>) -> Vec<i32> {
    let mut res: Vec<i32> = Vec::new();

    for i in 0..nums.len() {
        res.insert(index[i] as usize, nums[i]);
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_target_array() {
        let my_nums: Vec<i32> = vec![0, 1, 2, 3, 4];
        let my_index: Vec<i32> = vec![0, 1, 2, 2, 1];
        let test_vec: Vec<i32> = vec![0, 4, 1, 3, 2];

        assert_eq!(get_create_target_array(my_nums, my_index), test_vec);
    }
}
