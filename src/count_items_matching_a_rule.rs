pub fn get_count_items_matching_a_rule(
    items: Vec<Vec<String>>,
    rule_key: String,
    rule_value: String,
) -> i32 {
    let mut res = 0;

    for item in items.iter() {
        match rule_key.as_str() {
            "type" => {
                if item[0].eq(&rule_value) {
                    res += 1;
                }
            }
            "color" => {
                if item[1].eq(&rule_value) {
                    res += 1;
                }
            }
            "name" => {
                if item[2].eq(&rule_value) {
                    res += 1;
                }
            }
            _ => (),
        }
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_items_matching_a_rule() {
        let my_vec: Vec<Vec<String>> = vec![
            ["phone".to_string(), "blue".to_string(), "pixel".to_string()].to_vec(),
            [
                "computer".to_string(),
                "silver".to_string(),
                "phone".to_string(),
            ]
            .to_vec(),
            [
                "phone".to_string(),
                "gold".to_string(),
                "iphone".to_string(),
            ]
            .to_vec(),
        ];

        assert_eq!(
            get_count_items_matching_a_rule(my_vec, "type".to_string(), "phone".to_string()),
            2
        );
    }
}
