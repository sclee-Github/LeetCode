pub fn richest_customer_wealth(accounts: Vec<[i32; 3]>) -> i32 {
    let res: i32 = accounts.iter().map(|x| x.iter().sum()).max().unwrap();

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn richest_customer() {
        let my_vec: Vec<[i32; 3]> = vec![[2, 8, 7], [7, 1, 3], [1, 9, 5]].to_vec();

        assert_eq!(richest_customer_wealth(my_vec), 17);
    }
}
