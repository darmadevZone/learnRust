pub mod sample {
    pub fn sample_1_say() {
        pri
    }

    pub mod sample_2 {}
    pub mod sample_3 {
        pub mod sample_module_1 {
            pub fn sub1_test_fn_1() {
                println!("Hello, sub1_module1_fn1");
            }
            pub fn return_sub1_test_fn() -> i32 {
                1000
            }
            fn sub1_test_fn_2() {
                println!("Hello, sub1_module1_fn2");
            }
        }
        pub mod sample_module_2 {
            pub mod sub_module2 {
                pub fn sub2_test_fn_1() {
                    println!("Hello, sub2_module2_fn1");
                }
                fn sub2_test_fn_2() {
                    println!("Hello, sub2_module2_fn1");
                }
            }
        }
    }
}

pub struct Test_Sample_struct {
    pub val1: i32,
    pub val2: i32,
}
impl Test_Sample_struct {
    pub fn new(val1: i32, val2: i32) -> Test_Sample_struct {
        Test_Sample_struct { val1, val2 }
    }
}
