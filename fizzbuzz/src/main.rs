use core::num;

fn main() {
    //    fizzbuzzQ1(15);
    //   fizzbuzzQ2(20);
    fizzbuzzQ3(20)
}
fn fizzbuzzQ1(end: i32) {
    //AnsQ1 with while if
    let mut x: i32 = 1;
    while x <= end {
        if (x % 3 == 0 && x % 5 == 0) {
            println!("fizzbuzz {}", x);
        } else if (x % 3 == 0) {
            println!("fizz {}", x);
        } else if (x % 5 == 0) {
            println!("buzz {}", x);
        } else {
            println!("{}", x);
        }

        x += 1
    }
}
fn fizzbuzzQ2(end: i32) {
    //AnsQ2 with while match
    for x in 1..=end {
        match x % 15 {
            0 => println!("{} fizzbuzz", x),
            //あまりを利用した倍数特定
            3 | 6 | 9 | 12 => println!("{} fizz", x),
            5 | 10 => println!("{} buzz", x),
            _ => println!("{} no matched", x),
        }
    }
}
fn fizzbuzzQ3(end: i32) {
    //AnsQ2 with match touple
    for x in 1..=end {
        match (x % 3, x % 5) {
            (0, 0) => println!("{} fizzbuzz", x),
            (0, _) => println!("{} fizz", x),
            (1 | 2, 0) => println!("{} buzz", x),
            _ => println!("{}", x),
        }
    }
}

//my Ans
/*{
    let input_Number = 3;
    while true {
        if (input_Number % 15 == 0) {
            println!("fizzbuzz");
        } else if (input_Number % 3 == 0) {
            println!("fizz")
        } else if (input_Number % 5 == 0) {
            println!("buzz")
        } else {
            println!("{}", input_Number)
        }
    }

} */

/*
fizzbuzz
- 3x fizz
- 5x buzz
- 15x fizzbuzz
- other inputNumber


*/
