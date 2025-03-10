// Topic: Implementing functionality with the impl keyword
//
// Requirements:
// * Print the characteristics of a shipping box
// * Must include dimensions, weight, and color
//
// Notes:
// * Use a struct to encapsulate the box characteristics
// * Use an enum for the box color
// * Implement functionality on the box struct to create a new box
// * Implement functionality on the box struct to print the characteristics

// * Use an enum for the box color
enum Color {
    Red,
    Blue,
    Green,
    Yellow,
}
impl Color {
    fn print(&self) {
        match self {
            Color::Red => println!("Red"),
            Color::Blue => println!("Blue"),
            Color::Green => println!("Green"),
            Color::Yellow => println!("Color::Yellow"),
        }
    }
}

struct Dimensions {
    width: f64,
    height: f64,
    depth: f64,
}
impl Dimensions {
    fn print(&self) {
        println!("width: {:?}", self.width);
        println!("height: {:?}", self.height);
        println!("depth: {:?}", self.depth);
    }
}

// * Use a struct to encapsulate the box characteristics
struct ShippingBox {
    dimensions: Dimensions,
    weight: f64,
    color: Color,
}

impl ShippingBox {
    fn new(dimensions: Dimensions, weight: f64, color: Color) -> Self {
        Self {
            dimensions,
            weight,
            color,
        }
    }
    fn print(&self) {
        self.dimensions.print();
        self.color.print();
        println!("weight {:?}", self.weight);
    }
}

fn main() {
    let small_dimension = Dimensions {
        width: 1.0,
        height: 10.0,
        depth: 10.0,
    };
    let small_box = ShippingBox::new(small_dimension, 5.0, Color::Red);
    small_box.print();
}
