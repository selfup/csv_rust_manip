extern crate fut;

use fut::*;

fn main() {
    let mut dup = create_duplicate_map();

    let csv_contents = read_file_into_vec("./fixtures/something.csv");

    println!("{}", &csv_contents.len());

    let new_contents: Vec<String> = csv_contents.iter()
        .map(|content| {
            sub(content, "2012", "2018")
        })
        .filter(|content| {
            let columns: Vec<&str> = content.split(",").collect();
            let is_dup = is_dup(&mut dup, columns[0]);

            content.contains("id,name,created_at") || !is_dup
        })
        .collect();

    write_file(new_contents.join("\n"), "./fixtures/result/test.csv");

    println!("{}", &new_contents.len())
}
