/// This is an exercise to test Trait, Generic Types.

mod shapes;

use shapes::shapes::{
    Triangle, Rectangle, Round
};
use shapes::calculable::AreaCalculable;

/// The main function will print 3 shapes' area one by one, which are all AreaCalculable.
fn main() {
    let shape_1 = Triangle { bot_edge: 4.0, height: 3.0, }; // Init a Triangle
    let shape_2 = Rectangle { width: 10.0, height: 8.0, };  // Init a Rectangle
    let shape_3 = Round { radius: 5.0 };    // Init a Round
    print_area(&shape_1);   // Print the area of the Triangle
    print_area(&shape_2);   // Print the area of the Rectangle.
    print_area(&shape_3);   // Print the area of the Round.
}

/// Print the area of a AreaCalculable Shape
fn print_area<T: AreaCalculable>(shape: &T) {
    println!("The area is: {:?}", shape.area());
}
