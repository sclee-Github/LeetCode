pub fn get_subtract(n: i32) -> i32 {
    let mut my_vec: Vec<i32> = Vec::new();

    for n in n.to_string().chars() {
        my_vec.push(n.to_digit(10).unwrap() as i32);
    }

    let product: i32 = my_vec.iter().product();
    let sum: i32 = my_vec.iter().sum();

    product - sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn subtract_product_and_sum_of_digits() {
        let n = 234;

        assert_eq!(get_subtract(n), 15);
    }
}
