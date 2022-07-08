pub fn if_cond(a: i32) {
    if a == 10 {//Did you notice? 'if' doesn't have parentheses
        println!("yeah! a is 10");
    } else if a < 5 {
        println!("yeah! a is not 10 but it is less than 5");
    } else {
        println!("a is {}", a);
    }

    let is_divisible_by_2 = if a % 2 == 0 { true } else { false };//tertiary way of doing if

    println!("a is {} and a is_divisible_by_2 {}", a, is_divisible_by_2);
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

pub fn loop_loop() {
    let a = [1, 2, 3, 5, 6];
    let mut cnt: usize = 0;

    loop {//like while loop without condition
        if a[cnt] % 3 == 0 {
            break;
        }
        cnt += 1;
    }
}