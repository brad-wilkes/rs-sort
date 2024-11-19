pub struct Sortstuff;

impl Sortstuff {
    pub fn bubble_sort(mut arr: Vec<i32>) -> Vec<i32> {
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