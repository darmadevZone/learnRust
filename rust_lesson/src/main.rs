use std::fmt::format;

pub mod touple;
//const B: i32 = 22;

fn main() {
    /*
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
    */

    //数値型
    /*
    i: 符号付き整数
    u: 符号なし整数
    f: 浮動小数
    */
    /*
    let a: i32 = -1;
    let b: f64 = 2.0;

    let c: u16 = 2;

    let f: f64 = 1 as f64 + 3.33;
    println!("{} {}{}{}", a, b, c, f)
    */

    // touple
    /*
    let t1: (i32, bool, f32) = (22, true, 4.4);
    println!("{:?}", t1);
    let i: f32 = t1.2;
    println!("{}", i)
     */

    // 配列
    /*
       let l1: [i16; 3] = [22, 22, 33];
       print!("{:?}", l1);
    */

    //ベクタ
    /*
    let v1: Vec<i32> = vec![1, 2, 3];
    let mut v3: Vec<i32> = Vec::new();
    v3.push(33);
    v3.push(322);
    v3.push(323332);
    println!("{:?}", v3);

    let x: Option<i32> = v3.pop();
    println!("{:?}", v3);
    println!("{:?}", x);

    let y: Option<&i32> = v3.get(1);
    println!("{:?}", y);
     */

    //文字型
    /*
    let c1: char = 'a';
    let c2: char = 'g';

    //文字列型
    let s1: &str = "rust";

    let s2: String = "Java".to_string();
    let s2: String = String::from("Python");

    let mut s4: String = String::from("Hello");
    s4.push_str(",rust");
    println!("{}", s4);

    println!("{}", s4 + ",golang");
    let s5: String = format!("{}{}", s1, "golang");
    println!("{}", s5);
     */
    say_hello();
    add(33, 33);
}

// function
//戻り値のない関数は空のタプルになる。

fn say_hello() {
    println!("say_hello")
}
fn add(a: i32, b: i32) -> i32 {
    a + b
}
