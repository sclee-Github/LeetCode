pub fn get_jewels_and_strons(jewels: String, stones: String) -> i32 {
    stones.chars().filter(|&c| jewels.contains(c)).count() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn jewels_and_stones() {
        let my_jewels = String::from("aA");
        let my_stones = String::from("aAAbbbb");

        assert_eq!(get_jewels_and_strons(my_jewels, my_stones), 3);
    }
}
