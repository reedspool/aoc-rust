use std::fs;

pub fn read_input(filename: &str) -> String {
    fs::read_to_string(format!("./inputs/{}.txt", filename))
        .expect("Something went wrong reading the file")
}
