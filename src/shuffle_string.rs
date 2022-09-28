use std::collections::HashMap;

pub fn get_shuffle_string(s: String, indices: Vec<i32>) -> String {
    let my_map: HashMap<_, _> = indices.iter().zip(s.chars()).collect();

    (0..(s.len() as i32))
        .map(|i| my_map.get(&i).unwrap())
        .collect::<String>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shuffle_string() {
        let my_string = String::from("codeleet");
        let my_vec: Vec<i32> = vec![4, 5, 6, 7, 0, 2, 1, 3];
        let test_string = String::from("leetcode");

        assert_eq!(get_shuffle_string(my_string, my_vec), test_string);
    }
}
