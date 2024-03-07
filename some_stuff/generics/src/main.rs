struct Point<T, E> {
    x: T,
    y: E,
}

impl<T, E> Point<T, E> {
    fn x(&self) -> &T {
        &self.x
    }

    fn y(&self) -> &E {
        &self.y
    }
}

fn main() {
    let p = Point { x: 5, y: "Not a number ;)" };

    println!("p.x = {}, p.y = {}", p.x(), p.y());
}