use std::rc::Rc;

fn smart_pointer() {
    //スマートポインタ
    let x: Box<i32> = Box::new(1);
    println!("log x: {:p}", x);
    // *xで参照外しができる。
    println!("*x + 2 = {}", *x + 2);

    //同じ値を参照することができる。
    let r: Rc<String> = Rc::new("Hello".to_string());
    println!("count1: {} pointer: {:p}", Rc::strong_count(&r), r);
    {
        let rr = Rc::clone(&r);
        println!("r: {:p}", r);
        println!("rr: {:p}", rr);
        println!("count: {} pointer: {:p}", Rc::strong_count(&r), r);
    }
}
