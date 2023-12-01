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

// Struct representing a triangle.
#[derive(Default, Debug, Clone)]
struct Triangle {
    a: f32,
    b: f32,
    c: f32,
    base: f32,
    height: f32,
    name: &'static str,
}

// Implementing methods for the Triangle struct.
impl Triangle {
    fn default() -> Self {
        Triangle {
            a: 1.0,
            b: 1.3,
            c: 1.5,
            base: 1.4,
            height: 1.6,
            name: "triangle1",
        }
    }
}

// Implementing the Shape trait for Triangle.
impl Shape for Triangle {
    fn new(dimensions: Vec<f32>, name: &'static str) -> Self {
        Triangle {
            a: dimensions.get(0).cloned().unwrap_or(0.0),
            b: dimensions.get(1).cloned().unwrap_or(0.0),
            c: dimensions.get(2).cloned().unwrap_or(0.0),
            base: dimensions.get(3).cloned().unwrap_or(0.0),
            height: dimensions.get(4).cloned().unwrap_or(0.0),
            name,
        }
    }

    fn area(&self) -> f32 {
        (0.5) * self.height * self.base
    }

    fn perimeter(&self) -> f32 {
        self.a + self.b + self.c
    }

    fn set_dimensions(&mut self, dimensions: Vec<f32>) {
        if let Some(val) = dimensions.get(0) {
            self.a = *val;
        }
        if let Some(val) = dimensions.get(1) {
            self.b = *val;
        }
        if let Some(val) = dimensions.get(2) {
            self.c = *val;
        }
        if let Some(val) = dimensions.get(3) {
            self.base = *val;
        }
        if let Some(val) = dimensions.get(4) {
            self.height = *val;
        }
    }

    fn get_dimensions(&self) -> &Vec<f32> {
        &vec![self.a, self.b, self.c, self.base, self.height]
    }

    fn set_name(&mut self, name: &'static str) {
        self.name = name;
    }

    fn get_name(&self) -> &str {
        self.name
    }
}

// Implementing comparison traits for Triangle.
impl PartialEq for Triangle {
    fn eq(&self, other: &Self) -> bool {
        self.perimeter() == other.perimeter()
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

impl PartialOrd for Triangle {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.perimeter().partial_cmp(&other.perimeter())
    }
}

impl From<&'static str> for Triangle {
    fn from(s: &'static str) -> Triangle {
        let dimensions: Vec<f32> = s
            .split(',')
            .filter_map(|val| val.parse().ok())
            .collect();

        Triangle {
            a: dimensions.get(0).cloned().unwrap_or(0.0),
            b: dimensions.get(1).cloned().unwrap_or(0.0),
            c: dimensions.get(2).cloned().unwrap_or(0.0),
            base: dimensions.get(3).cloned().unwrap_or(0.0),
            height: dimensions.get(4).cloned().unwrap_or(0.0),
            name: dimensions.get(5).cloned().unwrap_or(""),
        }
    }
}

// Function to run the triangle-related code.
pub fn run2() {
    // Creating a default Triangle with a specified name.
    let triangle1 = Triangle::default();

    // Displaying information about triangle1.
    println!(
        "Triangle 1 - Base: {}, Height: {}, Name: {}",
        triangle1.get_dimensions()[3],
        triangle1.get_dimensions()[4],
        triangle1.get_name()
    );

    // Creating a new Triangle with specific dimensions and a name.
    let triangle2 = Triangle::new(vec![2.3, 2.4, 3.6, 3.7, 3.8], "Triangle2");
    println!("{:?}", triangle2.get_dimensions());

    // Creating a new Triangle from a string representation.
    let triangle3 = Triangle::from("2.2,3.4,4.6,5.7,6.8,Triangle3");
    println!("{:?}", triangle3.get_dimensions());
}

// Main function to run the program.
fn main() {
    // Running the triangle-related code.
    run2();
}
