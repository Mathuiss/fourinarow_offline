use crate::views::console_view;
use std::io::stdin;

pub fn read_col() -> usize {
    loop {
        let mut buf = String::new();
        stdin().read_line(&mut buf).unwrap();
        buf = buf.replace("\r", "");
        buf = buf.replace("\n", "");
        let input: usize = match buf.parse() {
            Result::Ok(r) => r,
            Result::Err(_) => {
                console_view::print_msg("Please enter a column number between 1 and 7: ");
                continue;
            }
        };

        if 1 <= input && input <= 7 {
            return input;
        } else {
            console_view::print_msg("Please enter a column number between 1 and 7: ");
        }
    }
}
