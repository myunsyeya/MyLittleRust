use std::io::{self};
use std::str::FromStr;

/**
 * Read a line as String (trim trailing newline)
 */
fn read_line() -> String {
    let mut s = String::new();
    io::stdin().read_line(&mut s).expect("Failed to read line");
    s.trim_end().to_string()
}

/**
 * Read a single value from one line (e.g. i54, f54, etc.)
 */
fn read_one<T: FromStr>() -> T
where
    <T as FromStr>::Err: std::fmt::Debug,
{
    let line = read_line();
    T::from_str(line.trim()).expect("Failed to parse single value")
}

/**
 * Read multiple values from one line (space-separated)
 */
fn read_vec<T: FromStr>() -> Vec<T>
where
    <T as FromStr>::Err: std::fmt::Debug,
{
    let line = read_line();
    line.split_whitespace()
        .map(|x| T::from_str(x).expect("failed to parse value"))
        .collect()
}

/**
 * Read one line and split into characters
 * Note: `char` is a Unicode scalar value.
 */
fn read_chars() -> Vec<char> {
    let line = read_line();
    line.chars().collect()
}

fn main() {
    println!("Hello, world!");
}
