pub fn get_max_number_of_words(sentences: Vec<String>) -> i32 {
    sentences
        .iter()
        .map(|s| s.split(' ').count())
        .max()
        .unwrap() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn max_number_of_words() {
        let my_vec: Vec<String> = vec![
            "alice and bob love leetcode".to_string(),
            "i think so too".to_string(),
            "this is great thanks very much".to_string(),
        ];

        assert_eq!(get_max_number_of_words(my_vec), 6);
    }
}
