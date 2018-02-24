use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;

const READ_ERR: &str = "something went wrong reading the file";
const WRITE_ERR: &str = "something went wrong writing the file";
const NOT_FOUND: &str = "file not found";

fn main() {
    let csv_contents = stack_lines_into_vec("./fixtures/something.csv");
    let mut duplicates = create_duplicate_map();

    println!("{}", &csv_contents.len());

    let new_contents: Vec<String> = csv_contents.iter()
        .map(|content| {
            sub(content, "2012", "2018")
        })
        .filter(|content| {
            let columns: Vec<&str> = content.split(",").collect();
            let is_dup = is_duplicate(&mut duplicates, columns[0]);

            content.contains("id,name,created_at") || !is_dup
        })
        .collect();

    write_string_file(new_contents.join("\n"), "./fixtures/result/test.csv");

    println!("{}", &new_contents.len())
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

fn write_string_file(source: String, target: &str) {
    let mut file = File::create(target).expect(NOT_FOUND);
    file.write_all(source.as_bytes()).expect(WRITE_ERR);
}

fn sub(source: &String, pattern: &str, replacement: &str) -> String {
    String::from(str::replace(&source, pattern, replacement))
}

fn create_duplicate_map() -> HashMap<String, bool> {
    let dup_map: HashMap<String, bool> = HashMap::new();
    dup_map
}

fn is_duplicate(dup_map: &mut HashMap<String, bool>, id: &str) -> bool {
    match dup_map.get(id) {
        Some(_) => true,
        None => {
            dup_map.insert(String::from(id), true);
            false
        }
    }
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
