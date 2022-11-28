pub fn is_ugly(mut n: i32) -> bool {
    for p in vec![2, 3, 5].iter() {
        while n % p == 0 && n % p < n {
            n /= p;
        }
    }

    n == 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ugly_number() {
        let n = 6;

        assert_eq!(true, is_ugly(n));
    }
}
