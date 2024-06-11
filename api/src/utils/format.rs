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
