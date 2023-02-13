mod quicksort;
use std::fs;

fn read_file_string(path: &str) -> Result<String, Box<dyn std::error::Error>> {
    let data = fs::read_to_string(path)?;
    Ok(data)
}

fn main() {
    match read_file_string("./test/test_file.txt") {
        Ok(numbers) => {
            let mut num: Vec<i32> = numbers
                .split_whitespace()
                .map(|s| s.parse().expect("parse error"))
                .collect();
            // println!("{:?}", quicksort::solve(&mut num));
            quicksort::solve(&mut num);
            println!("{:?}", num);
        }
        Err(e) => println!("ERROR: {:?}", e),
    }
}
