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

#[test]
fn it_stacks_lines_into_vec() {
    let string_vec: Vec<String> = stack_lines_into_vec("./fixtures/test/test.csv");

    assert_eq!("hello,", string_vec[0]);
    assert_eq!("world,", string_vec[1]);
    assert_eq!("foo,", string_vec[2]);
    assert_eq!("bar", string_vec[3]);
}

#[test]
fn it_opens_file_as_a_string() {
    let string_file: String = open_file_as_string("./fixtures/test/test.csv");

    assert_eq!("hello,\nworld,\nfoo,\nbar\n", string_file);
}

#[test]
fn it_replaces_string_contents() {
    let string: String = "hello\nworld\nfoo\nbar".to_string();

    assert_eq!(String::from("helloworldfoobar"), sub(&string, "\n", ""));
}
