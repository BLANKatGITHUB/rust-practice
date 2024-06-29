struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn new(a: T) -> Point<T> {
        Point {
            x: a,
            y: b,
        }
    }
}

fn main() {
    let p1 = Point::new(5,6);
    println!("{}", p1.x);
}