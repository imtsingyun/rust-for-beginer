fn main() {
    // 作用域
    let s = String::from("hello");
    {
        let r = "world";
        println!("{}, {}", s, r);
    }
    // 此处 r 已经超出作用域
    // println!("{}, {}", s, r)
    println!("{}", s);

    // 可变
    let mut s = String::from("hello2");
    s.push_str(", world");
    println!("{}", s);
}
