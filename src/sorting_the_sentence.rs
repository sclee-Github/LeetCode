pub fn get_sorting_the_sentence(s: String) -> String {
    let mut my_vec: Vec<(i32, &str)> = Vec::new();

    for words in s.split_whitespace() {
        my_vec.push((
            words.split_at(words.len() - 1).1.parse::<i32>().unwrap(),
            words.split_at(words.len() - 1).0,
        ))
    }

    my_vec.sort_by_key(|words_key| words_key.0);

    my_vec
        .iter()
        .map(|words_value| words_value.1)
        .collect::<Vec<&str>>()
        .join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn soring_the_sentence() {
        let my_string = String::from("is2 sentence4 This1 a3");
        let test_string = String::from("This is a sentence");

        assert_eq!(get_sorting_the_sentence(my_string), test_string);
    }
}
