#![deny(clippy::all)]

fn main() {
    let name: &str = "foo";
    let age: u8 = 20;
    let num: i8 = 33;
    let mut copy_name = &name;
    let fruit = "banana";
    let ref_fruit = &fruit;
    let ref_ref_fruit = &&fruit;
    println!("name:{},copy_name:{}", name, copy_name);
    println!("age:{},num:{}", age, num);
    test()
}

fn test() {}
