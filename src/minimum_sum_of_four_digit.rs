pub fn minimum_sum_of_four_digit(num: i32) -> i32 {
    let my_vec: Vec<i32> = get_vec(num);

    my_vec[0] * 10 + my_vec[1] * 10 + my_vec[2] + my_vec[3]
}

pub fn get_vec(mut my_num: i32) -> Vec<i32> {
    let mut my_vec: Vec<i32> = Vec::new();

    while my_num > 0 {
        my_vec.push(my_num % 10);
        my_num /= 10;
    }

    my_vec.sort();
    my_vec
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn minimum_sum() {
        assert_eq!(minimum_sum_of_four_digit(2932), 52);
    }
}
