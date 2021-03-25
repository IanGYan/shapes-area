# README
Sample code to print area of shapes. \
The purpose of shapes-area is for learning to use trait, Generic Types in Rust. \
The program will create 3 kinds of shapes:
- a Triangle
- a Rectangle
- a Round

Every type of shapes implements a trait named AreaCalculable, which defines a area() fucntion.
The main() fuction pass the AreaCalculable Type to a function print_area(), which will print the area value of the specific type.

The repo is orgnized with mod.

If want to try in [playground](http://play.rust-lang.org), you also can use the simlified code as below:

```
fn main() {
    let shape_1 = Triangle { bot_edge: 4.0, height: 3.0, };
    let shape_2 = Rectangle { width: 10.0, height: 8.0, };
    let shape_3 = Round { radius: 5.0 };
    print_area(&shape_1);
    print_area(&shape_2);
    print_area(&shape_3);
}

fn print_area<T: AreaCalculable>(shape: &T) {
    println!("The area is: {:?}", shape.area());
}

pub trait AreaCalculable {
    fn area(&self) -> f32;
}

pub struct Triangle {
    pub bot_edge: f32,
    pub height: f32,
}

impl AreaCalculable for Triangle {
    fn area(&self) -> f32 {
        self.bot_edge * self.height * 0.5
    }
}

pub struct Rectangle {
    pub width: f32,
    pub height: f32,
}

impl AreaCalculable for Rectangle {
    fn area(&self) -> f32 {
        self.width * self.height
    }
}

pub struct Round {
    pub radius: f32,
}

impl AreaCalculable for Round {
    fn area(&self) -> f32 {
        self.radius * self.radius * 3.1415926
    }
}
```
