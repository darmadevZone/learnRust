const B: i32 = 22;

fn main() {
    println!("Hello, world!");
    println!("Hello world");
    print!("Rust");
    println!("hello {}", "world");
    println!("Hi {}", "Student");

    //変数 定数
    let num: i32 = 1;
    println!("{}", num);
    let mut b: i32 = 22;
    b = 33;
    let b: i32 = 88;
    println!("{}", b);

    const A: i32 = 11;
    println!("{}", A);
    println!("{}", B);
}
