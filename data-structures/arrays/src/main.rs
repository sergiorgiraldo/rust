fn main() {
    linear_search();
    binary_search();
}

fn linear_search() {
    println!("###### linear search of array");
    let mut range = String::new();
    let mut key = String::new();
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
    let mut range = String::new();
    let mut key = String::new();
    let mut arr = Vec::new();

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
    return -1
}
