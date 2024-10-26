use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;

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

        Ok(bubble_sort(numbers))
    }
    fn bubble_sort(mut arr: Vec<i32>) -> Vec<i32> {
        let n = arr.len();
        let mut start = 0;
        let mut end = n - 1;

        while start < end {
            let mut swapped = false;

            // Forward pass
            for i in start..end {
                if arr[i] > arr[i + 1] {
                    arr.swap(i, i + 1);
                    swapped = true;
                }
            }

            // If no swaps in forward pass, the array is sorted
            if !swapped {
                break;
            }

            // Backward pass
            swapped = false;
            for i in (start..end).rev() {
                if arr[i] > arr[i + 1] {
                    arr.swap(i, i + 1);
                    swapped = true;
                }
            }

            // If no swaps in backward pass, the array is sorted
            if !swapped {
                break;
            }

            // Optimizations
            start += 1;
            end -= 1;
        }

        arr
    }
}
