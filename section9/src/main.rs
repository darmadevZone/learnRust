use std::fmt::Debug;

use section9::sample_trait::{double_area, Circle, Rectangle, Shape};
fn main() {
    /*
     let rect = Rectangle {
        width: 4.0,
        height: 5.0,
    };
    let circle = Circle { radius: 5.0 };
    println!("{}", rect.calc_area());
    println!("{}", circle.calc_area());
    println!("{}", rect.default_something());
    println!("{}", circle.default_something());
    println!("{}", double_area(&rect));
    println!("{}", double_area(&circle));
    */

    /*
    println!("{:?}", (1, 2, 3));

    #[derive(Debug, PartialEq)]
    struct S {
        val1: i32,
        val2: i32,
    }
    // Debug を実装する or derive属性を使用する。
    /*
     impl S {
        pub fn Debug(&self) -> (i32, i32) {
            (self.val1, self.val2)
        }
    }
     */

    println!("logs: {:?}", S { val1: 1, val2: 2 });
    let s1 = S { val1: 1, val2: 3 };
    let s2 = S { val1: 1, val2: 2 };
    println!("logs s1: {:?}", s1);
    println!("logs s2: {:?}", s2);
    println!("{}", s1 == s2)
     */
    //    let num = max(2.0, 3.0);
    //    println!("{}", num)
    let num: Point<i32> = Point { x: 10, y: 33 };
    println!("{}", num.max())
}
struct Point<T> {
    x: T,
    y: T,
}

impl<T: PartialOrd + Debug> Point<T> {
    fn max(&self) -> &T {
        if self.x >= self.y {
            &self.x
        } else {
            &self.y
        }
    }
}
impl Point<i32> {
    fn min(&self) -> i32 {
        if self.x <= self.y {
            self.x
        } else {
            self.y
        }
    }
}

/*
fn max<T>(a: T, b: T) -> T
where
    T: PartialOrd + Debug,
{
    if a >= b {
        a
    } else {
        b
    }
}
*/

/*
トレイト = interface
- デフォルト実装: トレイトの中で処理を書き、それを実行させるか、上書きして処理を実行させる。


Derive属性
- meta data = アノテーション
- #[属性名]


*/
