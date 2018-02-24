use std::fs::File;
use std::io::prelude::*;

const READ_ERR: &str = "something went wrong reading the file";
const NOT_FOUND: &str = "file not found";

fn main() {
    let csv_contents = stack_lines_into_vec("./fixtures/something.csv");

    println!("{}", &csv_contents.len());

    let new_contents: Vec<String> = csv_contents.iter()
        .map(|content| {
            sub(content, "2012", "2018")
        })
        .filter(|content| {
            !content.contains(",")
        })
        .collect();

    println!("{}", new_contents.len())
}

fn stack_lines_into_vec(filename: &str) -> Vec<String> {
    let mut target = vec![];

    for line in open_file_as_string(filename).split("\n") {
        target.push(String::from(line));
    }

    target
}

fn open_file_as_string(filename: &str) -> String {
    let mut file = File::open(filename).expect(NOT_FOUND);
    let mut contents = String::new();
    
    file.read_to_string(&mut contents).expect(READ_ERR);
    
    contents
}

fn sub(source: &String, pattern: &str, replacement: &str) -> String {
    String::from(str::replace(&source, pattern, replacement))
}
