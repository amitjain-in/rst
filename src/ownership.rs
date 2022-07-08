// ** Golden rules of references in rust
// 1. At any given time, you can have either one mutable reference or any number of immutable references.
// 2. References must always be valid.

pub fn ptr() {
    //Uncomment below code and when you run see the magic of ownership
    // let mut s = String::from("Hello");
    // let copied = s;
    //
    // s.push_str(" World");
    // println!("Values of s: {} copied: {}", s, copied); // ^^^^^^^^^^^^^^^^^^^^ value borrowed here after move
}

pub fn pass_to_fn() {
    // let s = String::from("Push");
    // transfer_ownership_to_me(s); // value moved here.
    // println!("{}", s); //    ^ value borrowed here after move
}

fn transfer_ownership_to_me(s: String) {
    println!("{}", s);
}

pub fn pass_references() {
    let mut s = String::from("Push");
    print_fn_reference(&s);
    println!("Using s after passing reference to a function - {}", s);
    s.push_str(" and Pull");
    println!("After modification to s - {}", s);
}

fn print_fn_reference(s: &String) {
    println!("As a parameter to a function {}", s);
    //s.push_str(" and pull");  //Not allowed because s isn't mutable reference
}


pub fn pass_references_and_mutate() {
    let mut s = String::from("Push");
    print_fn_reference_and_mutate(&mut s);
    println!("Using s after passing reference to a function - {}", s);
    s.push_str(" and more Pull");
    println!("After modification to s - {}", s);//Should print Push and Pull and more Pull
}

fn print_fn_reference_and_mutate(s: &mut String) {
    println!("Below modification of referenced parameter {}", s);
    s.push_str(" and pull");
    println!("After modification of referenced parameter {}", s);
}

pub fn multiple_borrows() {
    //More than 1 borrow is prevented to avoid data races
    let mut s = String::from("Push");
    let first = &mut s;
    // let second = &mut s; //Compilation Error  ^^^^^^ second mutable borrow occurs here
    // println!("Second borrow {}", second);
    println!("First borrow {}", first);
}

pub fn mixed_references() {
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    // let r3 = &mut s; //Compilation Error You cannot mix mutable and immutable references within the same scope
    println!("{}, and {}", r1, r2);
    //println!("{}, {}, and {}", r1, r2, r3);

    let r3 = &mut s; //But now it's ok as r1 and r2 are out of scope
    println!("After r1 and r2 are out of scope you can use a mutable reference {}", r3);
}