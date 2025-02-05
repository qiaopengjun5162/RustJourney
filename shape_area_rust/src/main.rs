// 定义Shape trait，它有一个area方法
trait Shape {
    fn area(&self) -> f64;
}

// 为圆形实现Shape trait
struct Circle {
    radius: f64,
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius.powi(2)
    }
}

// 为三角形实现Shape trait
struct Triangle {
    base: f64,
    height: f64,
}

impl Shape for Triangle {
    fn area(&self) -> f64 {
        0.5 * self.base * self.height
    }
}

// 为正方形实现Shape trait
struct Square {
    side: f64,
}

impl Shape for Square {
    fn area(&self) -> f64 {
        self.side.powi(2)
    }
}

// 泛型函数，它接受实现了Shape trait的类型作为参数
fn print_area<T: Shape>(shape: &T) {
    let area = shape.area();
    println!("The area is: {}", area);
}

fn main() {
    // 创建圆形、三角形和正方形的实例
    let circle = Circle { radius: 5.0 };
    let triangle = Triangle {
        base: 3.0,
        height: 4.0,
    };
    let square = Square { side: 6.0 };

    // 调用print_area函数并传入不同的形状实例
    print_area(&circle);
    print_area(&triangle);
    print_area(&square);
}
