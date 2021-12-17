mod demo01;
mod demo02;

const MAX_POINTS: u32 = 100_000;

fn main() {

    let x = 5;
    // x = 6; // 报错
    // mut 使变量可变
    let mut y = 6;
    y = 9;
    println!("The value of x is {}", x);
}
