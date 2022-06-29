pub fn if_cond(a: i32) {
    if a == 10 {//Did you notice? 'if' doesn't have parentheses
        println!("yeah! a is 10");
    } else if a < 5 {
        println!("yeah! a is not 10 but it is less than 5");
    } else {
        println!("a is {}", a);
    }
}

pub fn for_loop() {
    let a = [1, 2, 3, 5, 6];
    println!("For loop using for . in .");
    for el in a {
        println!("{}", el);
    }

    println!("For loop using index");
    for i in 0..a.len() {
        println!("{}", a[i]);
    }
}

// pub fn loop_loop() {
//     let a = [1, 2, 3, 5, 6];
//     loop {
//
//     }
// }