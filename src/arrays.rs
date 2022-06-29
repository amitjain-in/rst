use std::io;

pub fn arrays_basic() {
    let a = [1, 2, 3, 4, 5];
    println!("Array size {} and first element is {}", a.len(), a[0]);

    //Another way to declare an array
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("Array size {} and first element is {}", a.len(), a[0]);

    //Another way
    let a = [3; 5]; //Means array of 5 elements filled with 3 at each index
    println!("Array size {} and first element is {}", a.len(), a[0]);
}

pub fn test_array_bounds() {
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!(
        "The value of the element at index {} is: {}",
        index, element
    );
}