
pub fn type_of_defaults() {
    let i = 5;
    let f = 5.0;
    let c = 'c';
    let s = "ring";
    let s1 = String::new();
    let b = true;
    print_type_of(&i);
    print_type_of(&f);
    print_type_of(&c);
    print_type_of(&s);
    print_type_of(&s1);
    print_type_of(&b);
}

pub fn tuples() {
    let t = (5, 6.0, 's');
    print_type_of(&t);
    println!("{} is the first element of the tuple and its type is {}", t.0, get_type(&t.0));
    println!("{} is the second element of the tuple and its type is {}", t.1, get_type(&t.1));
    println!("{} is the third element of the tuple and its type is {}", t.2, get_type(&t.2));

    let (x, y, z) = t;
    println!("single back from tuple {}", y);
}

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn get_type<T>(_: &T) -> &'static str {
    return std::any::type_name::<T>();
}

