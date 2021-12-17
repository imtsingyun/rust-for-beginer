use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("猜数！");

    let secret_number = rand::thread_rng().gen_range(1..101);

    loop {
        println!("猜测一个数据");
        // let foo = 1;    // immutable
        // let bar = foo;  // immutable
        // mut 使变量可变
        let mut guess = String::new();

        // &mut 可变，& 取地址符号，引用传递，引用也是不可变的，&mut 是可变的引用
        io::stdin().read_line(&mut guess).expect("无法读取行");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };

        println!("你猜测的数字是：{}", guess);
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => print!("Too big"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
    println!("神秘数字是：{}", secret_number);
}
