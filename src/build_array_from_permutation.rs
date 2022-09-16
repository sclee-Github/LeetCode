pub fn build_array(nums: Vec<i32>) -> Vec<i32> {
    let nums_iter = nums.iter();
    let mut ans: Vec<i32> = Vec::new();

    for &i in nums_iter {
        // println!("nums[{}] is {}", i, nums[i as usize]);
        ans.push(nums[i as usize]);
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build() {
        let vec: Vec<i32> = vec![0, 1, 2, 4, 5, 3];

        assert_eq!(build_array([0, 2, 1, 5, 3, 4].to_vec()), vec);
    }
}
