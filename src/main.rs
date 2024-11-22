mod sortstuff;

use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;
use sortstuff::Sortstuff;

fn main() {
    let file_path = "data.txt"; // where is the file stored?
    let sorted_numbers = read_and_sort_file(file_path);

    println!("Sorted numbers: {:?}", sorted_numbers);

    fn read_and_sort_file(file_path: &str) -> io::Result<Vec<i32>> {
        let path = Path::new(file_path);
        let file = File::open(&path)?;
        let reader = io::BufReader::new(file);

        let mut numbers = Vec::new();

        for line in reader.lines() {
            let line = line?;
            if let Ok(num) = line.parse::<i32>() {
                numbers.push(num);
            }
        }

        Ok(Sortstuff::bubble_sort(numbers))
    }

}
