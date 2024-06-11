pub fn num_to_alphabet(mut num: usize) -> String {
    let mut result = String::new();

    loop {
        let remainder = num % 26;
        result.insert(0, (b'A' + remainder as u8) as char);
        num = num / 26;

        if num == 0 {
            break;
        } else {
            num -= 1;
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_num_to_alphabet() {
        assert_eq!(num_to_alphabet(0), "A");
        assert_eq!(num_to_alphabet(1), "B");
        assert_eq!(num_to_alphabet(25), "Z");
        assert_eq!(num_to_alphabet(26), "AA");
        assert_eq!(num_to_alphabet(27), "AB");
        assert_eq!(num_to_alphabet(51), "AZ");
        assert_eq!(num_to_alphabet(52), "BA");
        assert_eq!(num_to_alphabet(53), "BB");
        assert_eq!(num_to_alphabet(77), "BZ");
        assert_eq!(num_to_alphabet(78), "CA");
    }
}
