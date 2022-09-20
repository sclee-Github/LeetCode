pub fn shuffle_the_array(nums: Vec<i32>, n: i32) -> Vec<i32> {
    let (first_slice, second_slice) = nums.split_at(n as usize);
    let mut ans_vec: Vec<i32> = Vec::new();

    for i in 0..n {
        ans_vec.push(first_slice[i as usize]);
        ans_vec.push(second_slice[i as usize]);
    }

    ans_vec
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shuffle() {
        let my_vec: Vec<i32> = vec![2, 5, 1, 3, 4, 7];
        let test_vec: Vec<i32> = vec![2, 3, 5, 4, 1, 7];

        assert_eq!(shuffle_the_array(my_vec, 3), test_vec);
    }
}
