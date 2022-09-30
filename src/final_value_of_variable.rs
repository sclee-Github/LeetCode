pub fn final_value_after_operations(operations: Vec<String>) -> i32 {
    operations.iter().fold(0, |res, oper| {
        if oper.contains("++") {
            res + 1
        } else {
            res - 1
        }
    })

    // for operation in my_operations {
    //     match operation.as_str() {
    //         "++X" | "X++" => res += 1,
    //         _ => res -= 1,
    //     }
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn final_value_of_variable() {
        let my_operations: Vec<String> =
            vec!["--X".to_string(), "X++".to_string(), "X++".to_string()];

        assert_eq!(final_value_after_operations(my_operations), 1);
    }
}
