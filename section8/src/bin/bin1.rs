fn main() {
    println!("this is bin1 file");
    section8::lib_say_hello();
}
/*
バイナリクレートfileでは、そのファイルだけで実行することができる。
default では、main.rsがそれにあたる。
- src/bin/ ... にファイルを作ることでバイナリクレートを作ることができる。

バイナリクレート間で共有することができない
ライブラリクレートは共有することができる。
*/
