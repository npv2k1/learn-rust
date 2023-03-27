// Khai báo một enum với các giá trị ShapeType
enum ShapeType {
    Circle,
    Square,
    Triangle,
}

// Khai báo struct Shape với thuộc tính shape_type
struct Shape {
    shape_type: ShapeType,
}

// Khai báo một trait với phương thức calculate_area
trait CalculateArea {
    fn calculate_area(&self) -> f64;
}

// Implement trait CalculateArea cho struct Shape
impl CalculateArea for Shape {
    fn calculate_area(&self) -> f64 {
        match self.shape_type {
            ShapeType::Circle => 3.14 * 10.0 * 10.0,
            ShapeType::Square => 20.0 * 20.0,
            ShapeType::Triangle => 0.5 * 20.0 * 30.0,
        }
    }
}

type TypeShape = Shape;

fn main() {
    // Khởi tạo một struct Shape với shape_type là Circle
    let shape = Shape {
        shape_type: ShapeType::Circle,
    };

    // Sử dụng trait CalculateArea cho struct Shape

    println!("Diện tích hình là: {}", <TypeShape>::calculate_area(&shape));
}
