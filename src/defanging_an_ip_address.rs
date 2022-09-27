pub fn get_defanging_an_ip_address(mut address: String) -> String {
    address = address.replace(".", "[.]");
    address
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defanging_an_ip_address() {
        let my_string = String::from("255.100.50.0");
        let test_string = String::from("255[.]100[.]50[.]0");

        assert_eq!(get_defanging_an_ip_address(my_string), test_string);
    }
}
