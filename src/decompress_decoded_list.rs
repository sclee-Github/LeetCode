pub fn get_decompreess_encoded_list(nums: Vec<i32>) -> Vec<i32> {
    let chunk_size = 2;
    let mut res_vec: Vec<i32> = Vec::new();

    for chunk in nums.chunks(chunk_size) {
        res_vec.extend(vec![chunk[1]; chunk[0] as usize]);
    }

    res_vec
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decompress_encoded_list() {
        let my_vec: Vec<i32> = vec![1, 2, 3, 4];
        let test_vec: Vec<i32> = vec![2, 4, 4, 4];

        assert_eq!(get_decompreess_encoded_list(my_vec), test_vec);
    }
}
