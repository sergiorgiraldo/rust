fn main() {
    linear_search();
    binary_search();
    selection_sort();
    bubble_sort();
    insertion_sort();
    pair_sum();
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

