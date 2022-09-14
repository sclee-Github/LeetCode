impl Solution {
    pub fn get_concatenation(mut nums: Vec<i32>) -> Vec<i32> {
        let nums_len = nums.len();

        for i in 0..nums_len {
            nums.push(nums[i]); // nums.extend([nums[i]]);
        }

        nums
    }
}
