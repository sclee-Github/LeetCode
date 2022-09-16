pub fn get_concatenation(mut nums: Vec<i32>) -> Vec<i32> {
    let nums_len = nums.len();

    for i in 0..nums_len {
        nums.push(nums[i]);
        // num.extend(nums[i]);
    }

    nums
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn concatenation() {
        let vec = vec![1, 2, 3, 1, 2, 3];

        assert_eq!(get_concatenation([1, 2, 3].to_vec()), vec);
    }
}

/* First Solution
impl Solution {
    pub fn get_concatenation(mut nums: Vec<i32>) -> Vec<i32> {
        let nums_len = nums.len();

        for i in 0..nums_len {
            nums.push(nums[i]); // nums.extend([nums[i]]);
        }

        nums
    }
}
*/
