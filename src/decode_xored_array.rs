pub fn get_decode_xored_array(encoded: Vec<i32>, first: i32) -> Vec<i32> {
    let mut res_vec: Vec<i32> = vec![first];

    for i in 0..encoded.len() {
        res_vec.push(encoded[i] ^ res_vec[i]);
    }

    res_vec
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decode_xored_array() {
        let my_vec: Vec<i32> = vec![6, 2, 7, 3];
        let test_vec: Vec<i32> = vec![4, 2, 0, 7, 4];

        assert_eq!(get_decode_xored_array(my_vec, 4), test_vec);
    }
}
