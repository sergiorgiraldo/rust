fn main() {
    linear_search();
    binary_search();
    selection_sort();
    bubble_sort();
    insertion_sort();
    pair_sum();
    wave_print();
    spiral_print();
    rotate_array();
    staircase_search();
}

fn build_array() -> Vec<u32> {
    let mut range = String::new();
    let mut arr = vec![];

    println!("How many elements you want to enter in array: ");
    std::io::stdin()
        .read_line(&mut range)
        .expect("Failed to read input.");
    let range: u32 = range.trim().parse().unwrap();

    println!("Enter array elements: ");
    for index in 0..range {
        let mut element = String::new();
        println!("Enter element {} ", index + 1);
        std::io::stdin()
            .read_line(&mut element)
            .expect("Failed to read input.");
        let element: u32 = element.trim().parse().unwrap();
        arr.push(element);
    }

    arr
}

fn build_array_2d(rows: usize, cols: usize) -> Vec<Vec<i32>> {
    let mut array = vec![vec![0; cols]; rows];
    let mut n = 0;

    for row in 0..rows {
        for col in 0..cols {
            n += 1;
            array[row][col] = n;
        }
    }
    array
}

fn print_array(arr: &Vec<Vec<i32>>, txt: &str) {
    let rows = arr.len();
    let cols = arr[0].len();
    println!("{}", txt);
    for row in 0..rows {
        for col in 0..cols {
            print!("{}\t", arr[row][col]);
        }
        println!("");
    }
}

fn linear_search() {
    println!("###### linear search of array");
    let mut key = String::new();
    let arr = build_array();

    //Ask for the element we want to search
    println!("Enter the key you want to search: ");
    std::io::stdin()
        .read_line(&mut key)
        .expect("Failed to read input.");
    let key: u32 = key.trim().parse().unwrap();

    // Find out the index of that element by traversing the array
    // Linear Search Algorithm
    let mut found = false;
    for (count, &item) in arr.iter().enumerate() {
        if item == key {
            println!("{} is present at index: {}", key, count);
            found = true;
        }
    }
    if !found {
        println!("{} is not present.", key);
    }
}

fn binary_search() {
    println!("###### linear search of array");
    let mut key = String::new();
    let mut arr = build_array();

    //Ask for the element we want to search
    println!("Enter the key you want to search: ");
    std::io::stdin()
        .read_line(&mut key)
        .expect("Failed to read input.");
    let key: u32 = key.trim().parse().unwrap();

    // Search
    arr.sort();
    let index = binary_search_algo(arr, key); // searching the value

    if index == -1 {
        println!("Key is not present");
    } else {
        println!("Key is present at {}", index);
    }
}

fn binary_search_algo(arr: Vec<u32>, key: u32) -> i32 {
    let range = arr.len();
    let mut start = 0;
    let mut end = range - 1;
    while start <= end {
        let mid = (start + end) / 2;
        if arr[mid] == key {
            return mid as i32;
        }

        // Search values that are greater than val - to right of current mid_index
        if arr[mid] < key {
            start = mid + 1;
        }

        // Search values that are less than val - to the left of current mid_index
        if arr[mid] > key {
            end = mid - 1;
        }
    }
    return -1;
}

/*
    Selection Sort
    Arrange a randomly shuffled array in increasing & descreasing order.

    The selection sort algorithm sorts an array by repeatedly finding the minimum element
    from unsorted part and putting it at the beginning of unsorted array.
    The algorithm maintains two subarrays in a given array.
    1) The subarray which is already sorted.
    2) Remaining subarray which is unsorted.
*/
fn selection_sort() {
    println!("###### selection sort");

    let mut arr: Vec<i32> = vec![4, 2, 9, 6, 23, 12, 34, 0, 1];
    let range = arr.len();
    println!("Original Array: {:?}", arr);

    for i in 0..range - 1 {
        let mut min_index = i;
        for j in i + 1..range {
            if arr[j] < arr[min_index] {
                min_index = j;
            }
        }

        let temp = arr[i];
        arr[i] = arr[min_index];
        arr[min_index] = temp;

        println!("Step {}: {:?}", i + 1, arr);
    }
    println!("Sorted Array: {:?}", arr);
}

/*
   Bubble Sort
   Take the largest element towards end! (Pairwise swapping)
*/
fn bubble_sort() {
    println!("###### bubble sort");

    let mut arr: Vec<i32> = vec![4, 2, 9, 6, 23, 12, 34, 0, 1];
    let range = arr.len();

    println!("Original Array: {:?}", arr);

    for i in 1..range {
        for j in 0..range - i {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
        println!("Step {}: {:?}", i, arr);
    }
    println!("Sorted Array: {:?}", arr);
}

/*
    Insertion Sort
    Insert the "current" element in right position
    The array is virtually split into a sorted and an unsorted part.
    Values from the unsorted part are picked and placed at the correct position in the sorted part.
*/
fn insertion_sort() {
    println!("###### insert sort");

    let mut arr: Vec<i32> = vec![4, 2, 9, 6, 23, 12, 34, 0, 1];
    let range = arr.len();

    println!("Original Array: {:?}", arr);

    for i in 1..range {
        let curr = arr[i];
        let mut j = i;
        while j > 0 && arr[j - 1] > curr {
            arr[j] = arr[j - 1];
            j -= 1;
        }
        arr[j] = curr;
        println!("Step {}: {:?}", i, arr);
    }

    println!("Sorted Array: {:?}", arr);
}

/*
   Pair Sum Problem (Two Pointer Approach)
   Given a sorted array, Find pair of elements that sum to k(Given)
*/
fn pair_sum() {
    println!("###### pair sum, sum of 16");

    let arr = [1, 3, 4, 5, 7, 10, 11, 12, 13];
    println!("Original Array: {:?}", arr);

    let k = 16;

    let mut i = 0;
    let mut j = arr.len() - 1;

    while i < j {
        let current_sum = arr[i] + arr[j];
        if current_sum == k {
            println!("{} and {}", arr[i], arr[j]);
            i += 1;
            j -= 1;
        } else if current_sum > k {
            j -= 1;
        } else {
            i += 1;
        }
    }
}

/*
Case1:
    Enter Array Rows & Cols: 3 3
    1 2 3
    4 5 6
    7 8 9
    Wave Print: 1 4 7  8 5 2  3 6 9
Case2:
    Enter Array Rows & Cols: 3 6
     1  2  3  4  5  6
     7  8  9 10 11 12
    13 14 15 16 17 18
    Wave Print: 1 7 13  14 8 2  3 9 15  16 10 4  5 11 17  18 12 6
*/
fn wave_print() {
    println!("###### wave print");

    let array = build_array_2d(3, 3);
    wave_print_algorithm(array);

    let array = build_array_2d(3, 6);
    wave_print_algorithm(array);
}

fn wave_print_algorithm(arr: Vec<Vec<i32>>) {
    let rows = arr.len();
    let cols = arr[0].len();

    print_array(&arr, "ORIGINAL");

    println!("WAVE");
    for col in 0..cols {
        if col % 2 == 0 {
            // Even Cols (Top Down Direction)
            for row in 0..rows {
                print!("{} ", arr[row][col]);
            }
        } else {
            // Odd Cols (Bottom Up Direction)
            for row in (0..rows).rev() {
                print!("{} ", arr[row][col]);
            }
        }
    }
    println!("");
}

/*
OUTPUT:
Case1:
    Enter Rows & Cols: 4 4
     1  2  3  4
     5  6  7  8
     9 10 11 12
    13 14 15 16
    Spiral Pattern: 1 2 3 4   8 12 16   15 14 13   9 5   6 7   11   10
Case2:
    Enter Rows & Cols: 3 6
     1  2  3  4  5  6
     7  8  9 10 11 12
    13 14 15 16 17 18
    Spiral Pattern: 1 2 3 4 5 6   12 18   17 16 15 14 13   7   8 9 10 11
*/
fn spiral_print() {
    println!("###### spiral print");

    let array = build_array_2d(4, 4);
    spiral_print_algorithm(array);

    let array = build_array_2d(3, 6);
    spiral_print_algorithm(array);
}

fn spiral_print_algorithm(arr: Vec<Vec<i32>>) {
    let rows = arr.len();
    let cols = arr[0].len();

    print_array(&arr, "ORIGINAL");

    println!("SPIRAL");
    let mut start_row = 0;
    let mut start_col = 0;
    let mut end_row = rows - 1;
    let mut end_col = cols - 1;

    while start_row <= end_row && start_col <= end_col {
        // 1. First Row
        for i in start_col..=end_col {
            print!("{} ", arr[start_row][i]);
        }
        start_row += 1;
        if start_row <= end_row {
            print!("• ");
        }
        // 2. Last Col
        for i in start_row..=end_row {
            print!("{} ", arr[i][end_col]);
        }
        end_col -= 1;
        if start_row <= end_row {
            print!("• ");
        }

        // 3. Last Row
        if start_row <= end_row {
            for i in (start_col..=end_col).rev() {
                print!("{} ", arr[end_row][i]);
            }
            end_row -= 1;
            if start_row <= end_row {
                print!("• ");
            }
        }

        // 4. First Col
        if start_col <= end_col {
            for i in (start_row..=end_row).rev() {
                print!("{} ", arr[i][start_col]);
            }
            start_col += 1;
            if start_row <= end_row {
                print!("• ");
            }
        }
    }
    println!("");
}

fn rotate_array() {
    println!("###### rotate array");

    let mut arr = build_array_2d(4, 4);
    print_array(&arr, "ORIGINAL");

    let rows = arr.len();
    let cols = arr[0].len();

    // reverse each row
    for row in 0..rows {
        let (mut start_col, mut end_col) = (0, cols - 1);
        while start_col < end_col {
            arr[row].swap(start_col, end_col);
            start_col += 1;
            end_col -= 1;
        }
    }
    // transpose
    let mut transposed = (0..arr[0].len()).map(|_| vec![]).collect::<Vec<_>>();

    for original_row in arr.clone() {
        for (item, transposed_row) in original_row.into_iter().zip(&mut transposed) {
            transposed_row.push(item);
        }
    }

    println!();

    print_array(&transposed, "ROTATED");
}

/*
Search element in a row-wise & column-wise sorted array
*/
fn staircase_search() {
    println!("###### rotate array");

    let arr = build_array_2d(6, 6);
    print_array(&arr, "ORIGINAL");

    let rows = arr.len();
    let cols = arr[0].len();

    let mut key = String::new();

    println!("Enter the key you want to search: ");
    std::io::stdin()
        .read_line(&mut key)
        .expect("Failed to read input.");
    let key: i32 = key.trim().parse().unwrap();

    let mut row_start: i32 = 0;
    let mut col_end: i32 = (cols - 1) as i32;
    let mut success = false;

    while row_start <= (rows - 1) as i32 && col_end >= 0 {
        if key == arr[row_start as usize][col_end as usize] {
            success = true;
            break;
        } else if key > arr[row_start as usize][col_end as usize] {
            row_start += 1;
        } else {
            col_end -= 1;
        }
    }
    if success {
        println!("Values found at Index: {} , {}", row_start, col_end);
    } else {
        println!("Values Not Found");
    }
}
