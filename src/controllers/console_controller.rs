use std::io::stdin;

pub fn read_col() -> u8 {
    loop {
        let mut buf = String::new();
        stdin().read_line(&mut buf).unwrap();
        let input: u8 = buf
            .replace("\r\n", "")
            .parse()
            .expect("Please input a number between 1 and 7");

        if 1 <= input && input <= 7 {
            return input;
        }
    }
}
