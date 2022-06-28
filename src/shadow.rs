
pub fn shadowing() {
    println!("\n\n** shadowing");
    let a = 5;
    {
        let a = a + 1;//shadows earlier scope
        println!("In loop - a: {}", a); //6
    }
    println!("Outside loop - a: {}", a); //5
}

pub fn shadowing_diff_types() {
    println!("\n\n** shadowing_diff_types");
    let a = " Plank ";
    {
        let a = a.len();
        println!("In loop - a: {}", a); //7
    }
    println!("Outside loop - a: {}", a); //  Plank
}
