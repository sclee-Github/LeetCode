pub fn get_greatest_number_of_candies(candies: Vec<i32>, extra_candies: i32) -> Vec<bool> {
    let max_candies = candies.iter().max().unwrap();

    // println!("{:?}", my_vec.iter().map(|x| *x + my_extra >= *max_value).collect::<Vec<bool>>());
    candies
        .iter()
        .map(|x| x + extra_candies >= *max_candies)
        .collect()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn kids_with_greatest_number_of_candies() {
        let my_vec: Vec<i32> = vec![2, 3, 5, 1, 3];
        let my_extra: i32 = 3;
        let res: Vec<bool> = vec![true, true, true, false, true];

        assert_eq!(get_greatest_number_of_candies(my_vec, my_extra), res);
    }
}
