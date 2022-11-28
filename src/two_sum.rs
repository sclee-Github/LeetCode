use ::std::collections::HashMap;

pub fn get_two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut my_hashmap: HashMap<i32, i32> = HashMap::new();

    for i in 0..nums.len() {
        match my_hashmap.get(&nums[i]) {
            Some(&x) => return vec![x, i as i32],
            None => my_hashmap.insert(target - nums[i], i as i32),
        };
    }

    return vec![-1, -1];
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn two_sum() {
        let nums = vec![3, 2, 4];
        let target = 6;

        assert_eq!(vec![1, 2], get_two_sum(nums, target));
    }
}
