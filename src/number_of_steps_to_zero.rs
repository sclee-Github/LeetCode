pub fn number_of_steps(mut num: i32) -> i32 {
    let mut count = 0;

    while num > 0 {
        match num & 1 {
            0 => num >>= 1,
            _ => num ^= 1,
        }
        count += 1;
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn number_of_steps_to_zero() {
        let my_num = 14;

        assert_eq!(number_of_steps(my_num), 6);
    }
}
