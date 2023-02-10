pub fn is_valid_ip(ip: &str) -> bool {
    (ip.split(".").collect::<Vec<&str>>().len() == 4
        && ip
            .split(".")
            .filter(|c| {
                c.chars().all(|x| x.is_numeric())
                    && c.parse::<u8>().is_ok()
                    && (*c.as_bytes().get(0).unwrap() != b'0' || *c == "0")
            })
            .collect::<Vec<&str>>()
            .len()
            == 4)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_valid_ip_test() {
        assert!(is_valid_ip("0.0.0.0"));
        assert!(is_valid_ip("12.255.56.1"));
        assert!(is_valid_ip("137.255.156.100"));

        assert!(!is_valid_ip(""));
        assert!(!is_valid_ip("1.2.3.4."));
        assert!(!is_valid_ip("abc.def.ghi.jkl"));
        assert!(!is_valid_ip("123.456.789.0"));
        assert!(!is_valid_ip("12.34.56"));
        assert!(!is_valid_ip("01.02.03.04"));
        assert!(!is_valid_ip("256.1.2.3"));
        assert!(!is_valid_ip("1.2.3.4.5"));
        assert!(!is_valid_ip("123,45,67,89"));
        assert!(!is_valid_ip("1e0.1e1.1e2.2e2"));
        assert!(!is_valid_ip(" 1.2.3.4"));
        assert!(!is_valid_ip("1.2.3.4 "));
        assert!(!is_valid_ip("12.34.56.-7"));
        assert!(!is_valid_ip("1.2.3.4\n"));
        assert!(!is_valid_ip("\n1.2.3.4"));
    }
}
