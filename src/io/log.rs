use std::io::{self, Write};

pub fn log(s: &str) {
    let mut s2 = s.to_owned();
    s2.push_str("\n");
    io::stdout().write_all(s2.as_bytes()).unwrap();
}
