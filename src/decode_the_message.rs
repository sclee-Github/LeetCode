use std::collections::hash_map::Entry;
use std::collections::HashMap;

pub fn get_decode_the_message(mut key: String, message: String) -> String {
    let mut my_map = HashMap::new();
    let mut eng_value = b'a';

    key = key.split_whitespace().collect::<String>();

    for c in key.chars() {
        if let Entry::Vacant(o) = my_map.entry(c) {
            o.insert(eng_value as char);
            eng_value += 1;
        }
    }

    // for c in key.chars() {
    //     if my_map.contains_key(&c) {
    //         continue;
    //     }
    //     my_map.insert(c, eng_value as char);
    //     eng_value += 1;
    // }

    message
        .chars()
        .map(|c| my_map.get(&c).unwrap_or(&' '))
        .collect::<String>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decode_the_message() {
        let my_key = String::from("the quick brown fox jumps over the lazy dog");
        let my_message = String::from("vkbs bs t suepuv");
        let test_message = String::from("this is a secret");

        assert_eq!(get_decode_the_message(my_key, my_message), test_message);
    }
}
