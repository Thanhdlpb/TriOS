use std::io::{self, Write};

pub fn in_ra(chuoi: &str) {
    print!("{}", chuoi);
    io::stdout().flush().unwrap();
}

pub fn in_dong(chuoi: &str) {
    println!("{}", chuoi);
}

pub fn doc_dong() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}
