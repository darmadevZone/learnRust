struct Rectangle {
    width: u32,
    height: u32,
}
impl Rectangle {
    fn area(&self) -> u32 {
        let area = self.width * self.height;
        return area;
    }
    fn new(width: u32, height: u32) -> Rectangle {
        Rectangle {
            width: width,
            height: height,
        }
    }
}
enum Shape {
    Circle(u32),
    Square { width: u32, height: u32 },
    Triangle { width: u32, height: u32 },
}
impl Shape {
    fn say_hello(&self) {
        println!("Hello World");
    }
    fn triangle_area(&self, width: u32, height: u32) -> u32 {
        width * height / 2
    }
}
fn main() {
    /*
    let rectangle: Rectangle = Rectangle {
        width: 10,
        height: 10,
    };
    println!("log: {}", rectangle.height);
    println!("log: {}", rectangle.width);
    let area: u32 = rectangle.area();
    println!("{}", area)
    */
    //constructor
    let rectangle: Rectangle = Rectangle::new(20, 20);

    //enum
    /*
    let cricle: Shape = Shape::Circle(22);
    let square: Shape = Shape::Square {
        width: 20,
        height: 20,
    };
    let triangle: Shape = Shape::Triangle {
        width: 30,
        height: 30,
    };

    {
        cricle.say_hello();
        triangle.say_hello();
    }
    let trianglearea = triangle.triangle_area(20, 10);
    println!("{}", trianglearea)
    */

    //Option
    let a = Some(1);
    let b = Option::Some("Option<String>".to_string());
    let c: Option<i32> = None;

    let v = vec![1, 2, 3];
    let val = v.get(2);
    match val {
        // Some(1) => println!("{:?}", val),
        Some(x) if *x == 1 => println!("x is 1=={}", x),
        Some(x) => println!("x is not 1. and value exists {}", x),
        None => println!("value is None"),
    }
    if let Some(x) = val {
        println!("log: {}", x)
    }
}

/*
型関連関数
- struct:コンストラクタとして使うことがある。

Option
中身は：
enum Option<T>{
    None,
    Some(T),
}
*/
