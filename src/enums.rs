use std::fmt;
use std::fmt::{Formatter, write};
use crate::enums::Shape::{Rectangle, Square};

enum Colour {
    RED,
    GREEN,
    BLUE,
}

impl fmt::Display for Colour {
    //If you want use printable values for your enums, too much work for this I think
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match *self {
            Colour::RED => write!(f, "RED"),
            Colour::GREEN => write!(f, "GREEN"),
            Colour::BLUE => write!(f, "BLUE")
        }
    }
}

struct Car {
    colour: Colour,
    manufacturer: String,
}

pub fn enum_sample() {
    let red = Colour::RED;//red is type Colour
    println!("Red is blue {}", matches!(red, Colour::BLUE));// You cannot use == operator for comparison
    println!("Red is blue {}", matches!(red, Colour::RED));
    let red_safari = Car {
        colour: red,
        manufacturer: String::from("Tata Safari"),
    };

    println!("The future car is {} with colour {}", red_safari.manufacturer, red_safari.colour);
}

enum Shape {
    Square { side: i32 },
    Rectangle { width: i32, height: i32 },
    Box { width: i32, height: i32, depth: i32 },
    Circle { radius: i32 },
}

pub fn more_enum_sample() {
    let square = Square {
        side: 20
    };

    let rectangle = Rectangle {
        width: 30,
        height: 40,
    };
}