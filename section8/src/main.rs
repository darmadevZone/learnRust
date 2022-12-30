use rand::Rng;
mod first_module;
mod test_dir_module;

mod test_module {
    pub mod sub_module1 {
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
    pub mod sub_module2 {
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
use self::test_module::sub_module1;
// use self::test_module::sub_module1::return_sub1_test_fn;
fn main() {
    /*
     let random_num = rand::thread_rng().gen_range(1..100);
    println!("log: {}", random_num);

    //abosolute path
    crate::test_module::sub_module1::sub1_test_fn_1();
    crate::test_module::sub_module2::sub_module2::sub2_test_fn_1();
    //relative path
    self::test_module::sub_module1::sub1_test_fn_1();
    let num: i32 = self::test_module::sub_module1::return_sub1_test_fn();
    let use_num = self::sub_module1::return_sub1_test_fn();
    println!("log num:{}", num);
    println!("log use_num:{}", use_num);
    */

    self::first_module::first::first_module_say_comment();
    self::first_module::second::second_module_say_comment();
    self::test_dir_module::many_fns::many_fns_module::many_fns_say_hello();
}
/*
クレート: ライブラリ、実行ファイルテストツール全てが収められる。
- バイナリクレート、ライブラリクレートがある。
    - バイナリクレート: 実行が可能
    - ライブラリクレート: バイナリクレートから機能として呼び出されることで実行されるクレート

パッケージ: ある機能群を提供する1つ以上のクレーとの集合

モジュール: クレート内のコードをグループ化して、加速生と再利用生を高めるための機構
- 関数、構造体、列挙型、定数を稼働区政を上げるために分ける。
- モジュール内はデフォルトで、privateになる。pubでpublicになる。pub(super):親モジュールからアクセス可能
- path file...::file...::file...
** package > クレート

cargoは、
src/main.rs -> バイナリクレート
src/lib.rs -> ライブラリクレート
- src/binのdirに.rs fileを置くことで。それぞれのファイルが別のバイナリクレートになる。
- create.ioからクレートを取得するには、Cargo.tomlの[dependencies]に記載する。

moduleのきりわけかたは、
- main.rs と同じ階層にfileを作る
- dirを作って作るときは、mod.rs fileを作って、その中に作りたいモジュールを作る
*/
