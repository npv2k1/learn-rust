#[derive(Debug, Copy, Clone)]
struct Point {
    x: f64,
    y: f64,
}

impl Point {
    pub fn length (&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    pub fn distance (&self, other: &Point) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        (dx * dx + dy * dy).sqrt()
    }
}



fn main() {
    let a = Point { x: 3.0, y: 4.0 };
    println!("length of a is {}", a.length());
    let b = Point { x: 5.0, y: 12.0 };
    println!("distance between a and b is {}", a.distance(&b));
    println!("point a: {:?}", a);
}
