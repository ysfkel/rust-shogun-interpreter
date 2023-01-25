use std::collections::HashMap;
fn main() {
    let x: char = '=';

    let s: String = String::from("hello");
    let mut s = HashMap::new();

    s.insert("yusuf", 100);

    let d = s.get("yusuf").unwrap();
    println!("{}", d);

    let my = String::from("hellow");
    let a: usize = 1;
    // let b: i32 = 2;
    // let z = &my[a..b];
    // let m: &str = &"hello"[1..2];
    let v = '5';
    let r = '9' <= v;
    println!("set {}", r);

    let r = '9' <= v;
    let n = 'A' as u32;

    println!("equal s {}", n);

    let x = '9'.is_digit(10);
    println!("is dig {}", x)
}

fn is_digit(ch: char) -> bool {
    '0' <= ch && ch <= '9'
}
