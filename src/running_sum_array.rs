pub fn running_sum_array(mut nums: Vec<i32>) -> Vec<i32> {
    let nums_len = nums.len();
    let mut ans: Vec<i32> = vec![0; nums_len];

    for i in (0..nums_len).rev() {
        ans[i] = nums.iter().sum();
        nums.pop();
    }

    ans
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn running_sum() {
        let vec: Vec<i32> = vec![1, 2, 3, 4];
        let test_vec: Vec<i32> = vec![1, 3, 6, 10];

        assert_eq!(running_sum_array(vec), test_vec);
    }
}
