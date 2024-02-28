struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rectangle1 = Rectangle {
        width: 20,
        height: 20,
    };

    println!("{}", rectangle1.area())
}