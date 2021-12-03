use std::{
    fs,
    io::{self, BufRead},
};

#[macro_export]
macro_rules! get_input {
    ($parse_func:expr) => {
        aoc_2021::get_input_func(file!(), $parse_func)
    };
}

pub fn get_input_func<T>(file_name: &str, parse_func: impl Fn(String) -> T) -> Vec<T> {
    let f = fs::File::open(format!(
        "input\\{}",
        file_name
            .strip_prefix("src\\bin\\")
            .unwrap()
            .strip_suffix(".rs")
            .unwrap()
    ))
    .unwrap();
    let mut input: Vec<T> = vec![];
    let mut reader = io::BufReader::new(f);
    let mut buffer = String::new();

    while reader.read_line(&mut buffer).unwrap() != 0 {
        input.push(parse_func(buffer.trim().to_string()));
        buffer.clear();
    }
    input
}
