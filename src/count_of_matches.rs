pub fn number_of_matches(mut n: i32) -> i32 {
    let mut count = 0;

    while n > 1 {
        count += n / 2;

        if n % 2 == 0 {
            n = n / 2;
        } else {
            n = (n / 2) + 1;
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_of_matches() {
        let my_num = 7;

        assert_eq!(number_of_matches(my_num), 6);
    }
}
