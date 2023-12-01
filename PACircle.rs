#![allow(dead_code)] // Suppresses warnings about unused code.

use core::cmp::Ordering; // Importing a tool for comparing values.

// Defining a trait for geometric shapes.
trait Shape {
    fn new(dimensions: Vec<f32>, name: &'static str) -> Self;
    fn area(&self) -> f32;
    fn perimeter(&self) -> f32;
    fn set_dimensions(&mut self, dimensions: Vec<f32>);
    fn get_dimensions(&self) -> &Vec<f32>;
    fn set_name(&mut self, name: &'static str);
    fn get_name(&self) -> &str;
}

// Struct representing a circle.
#[derive(Default, Debug, Clone)]
struct Circle {
    radius: f32,
    name: &'static str,
}

// Implementing methods for the Circle struct.
impl Circle {
    fn default() -> Self {
        Circle {
            radius: 1.0,
            name: "Circle1",
        }
    }
}

// Implementing the Shape trait for Circle.
impl Shape for Circle {
    fn new(dimensions: Vec<f32>, name: &'static str) -> Self {
        Circle {
            radius: dimensions.get(0).cloned().unwrap_or(0.0),
            name,
        }
    }

    fn area(&self) -> f32 {
        std::f32::consts::PI * self.radius * self.radius
    }

    fn perimeter(&self) -> f32 {
        2.0 * std::f32::consts::PI * self.radius
    }

    fn set_dimensions(&mut self, dimensions: Vec<f32>) {
        if let Some(radius) = dimensions.get(0) {
            self.radius = *radius;
        }
    }

    fn get_dimensions(&self) -> &Vec<f32> {
        &vec![self.radius]
    }

    fn set_name(&mut self, name: &'static str) {
        self.name = name;
    }

    fn get_name(&self) -> &str {
        self.name
    }
}

// Implementing comparison traits for Circle.
impl PartialEq for Circle {
    fn eq(&self, other: &Self) -> bool {
        self.perimeter() == other.perimeter()
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

impl PartialOrd for Circle {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.perimeter().partial_cmp(&other.perimeter())
    }
}

impl From<&'static str> for Circle {
    fn from(s: &'static str) -> Circle {
        let dimensions: Vec<f32> = s
            .split(',')
            .filter_map(|val| val.parse().ok())
            .collect();

        Circle {
            radius: dimensions.get(0).cloned().unwrap_or(0.0),
            name: dimensions.get(1).cloned().unwrap_or(""),
        }
    }
}

// Function to run the circle-related code.
pub fn run() {
    // Creating a default Circle with a specified name.
    let circle1 = Circle::default();

    // Displaying information about circle1.
    println!("Circle 1 - Radius: {}, Name: {}", circle1.radius, circle1.name);

    // Creating a new Circle with specific dimensions and a name.
    let circle2 = Circle::new(vec![2.3], "Circle2");

    // Displaying information about circle2.
    println!(
        "Circle 2 - Radius: {}, Name: {}",
        circle2.get_dimensions()[0],
        circle2.get_name()
    );

    // Creating a new Circle from a string representation.
    let circle3 = Circle::from("2.2,Circle3");

    // Displaying information about circle3.
    println!(
        "Circle 3 - Radius: {}, Name: {}",
        circle3.get_dimensions()[0],
        circle3.get_name()
    );

    // Comparing circles based on perimeter.
    let result1 = circle1.partial_cmp(&circle2);
    println!("Comparison Result 1: {:?}", result1);

    // Checking if circle1 is less than or equal to circle2 in size.
    let result2 = circle1.le(&circle2);
    println!("Comparison Result 2: {:?}", result2);

    // Checking if circle2 is equal to circle3 in size.
    let result3 = circle2.eq(&circle3);
    println!("Comparison Result 3: {:?}", result3);

    // Checking if circle2 is not equal to circle3 in size.
    let result4 = circle2.ne(&circle3);
    println!("Comparison Result 4: {:?}", result4);
}

// Main function to run the program.
fn main() {
    // Running the circle-related code.
    run();
}
