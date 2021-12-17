fn shadowing_test() {
    let x = 5;
    let x = x + 5;

    // 用长度覆盖原来的值
    let spaces = "    ";
    let spaces = spaces.len();

    println!("The value of x is {}", x)
}