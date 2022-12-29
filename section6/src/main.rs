use std::{fmt::format, rc::Rc};

fn main() {
    //所有権と参照
    //    send_v()
    /*
     {
        let s1 = String::from("Hello");
        let s2 = String::from("Rust");
        let (s1, s2, s3) = function_send_v(s1, s2);
        //        let s3 = function_send_v(s1, s2);
        // println!("log:{:?}", s3);
        println!("log:{},{},{}", s1, s2, s3)
    }
    */
    //    function_ref_v();
    smart_pointer();
}
fn send_v() {
    {
        let v1: Vec<i32> = vec![1, 2, 3];
        println!("{:?}", v1);

        //参照渡し
        let mut v2 = v1;
        v2.push(33);
        println!("{:?}", v2);

        //値渡し
        v2.push(11);
        let mut v3 = v2.clone();
        println!("{:?}", v2);
        println!("{:?}", v3);
    }
    // Copyトレイト型では、自動的にあたい渡しになる。文字、数値、論理
    let mut num_v = (true, 11, "fjsdlfds");
    let mut num_v2 = num_v;
    println!("{:?}", num_v)
}

fn function_send_v(a: String, b: String) -> (String, String, String) {
    let c: String = format!("{}{}", a, b);
    (a, b, c)
}

fn function_ref_v() {
    //参照
    {
        //共有参照
        let x1: Vec<i32> = vec![1, 2, 3];
        let x2: &Vec<i32> = &x1;
        println!("log: {:?}", x1);
        println!("log: {:?}", x2);
    }
    {
        //可変参照
        let mut s1: String = String::from("Hello");
        let s2: String = String::from("Rust");
        println!("log: {}", s1);
        let mut ref_s1 = s1;
        println!("log: {}", ref_s1);
        ref_s1 = String::from("ref_Rust");
        println!("log: {}", ref_s1);
    }
}

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
    println!("count: {} pointer: {:p}", Rc::strong_count(&r), r);
}
/*
スタック領域
- ローカル変数や関数の引数などの一時データを格納
- コンパイル時にサイズが確定している必要がある。
- last in first Out のシンプルな構造
- アクセスが高速
- 格納容量：8MBまで

ヒープ領域
- 動的なデータを格納することができる。
- スタックと比べるとアクセスが低速
- プログラムが責任を持って確保と解放をしないといけない
- メモリ管理にコストがかかる

所有権
- rustがメモリ管理のために採用している。
- メモリ上に存在する値を変数が所有
- 所有権の移動や借用できる。
** 変数がスコープを抜ける際に値を破棄することでメモリの解放忘れを防ぐ -> メモリリーク
** スコープを抜けて破棄されるのは1回のみ -> 多重か言い方を防げる

参照
- 値を代入しても所有権が移動しない
- 参照の型は通常の方に＆記号がついて形
- ある値を参照を作ることを借用
- 参照に\ * をつけることで参照の実態にアクセス可能(参照外し)

参照の種類
- 共有参照
    - 参照先を読むことはできるが、変更は不可
    - 変数に&記号をつけることで作成

- 可変参照
    - 値の読み出しと書き込みが可能
    - 変数に&mutをつけることで作成


ライフタイム
- 参照は全てライフタイムを保持している。
- ダングリングポインタを防ぐため。 {}ブロックによる参照のエラー

スマートポインタ
*/
