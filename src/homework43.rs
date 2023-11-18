use std::clone::Clone;

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Rectangle {
    width: i32,
    height: i32,
}

trait Addable {
    fn add(&self, other: &Self) -> Self;
}

impl Addable for Point {
    fn add(&self, other: &Self) -> Self {
        let other = other.clone();
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Addable for Rectangle {
    fn add(&self, other: &Self) -> Self {
        let other = other.clone();
        Rectangle {
            width: self.width + other.width,
            height: self.height + other.height,
        }
    }
}

fn add_objects<T: Addable>(a: &T, b: &T) -> T {
    a.add(b)
}

pub fn hw43() {
    let point1 = Point { x: 10, y: 20 };
    let point2 = Point { x: 30, y: 40 };
    let rectangle1 = Rectangle { width: 50, height: 60 };
    let rectangle2 = Rectangle { width: 70, height: 80 };

    let result_point = add_objects(&point1, &point2);
    let result_rectangle = add_objects(&rectangle1, &rectangle2);

    println!("Point addition result: {:?}", result_point);
    println!("Rectangle addition result: {:?}", result_rectangle);
}