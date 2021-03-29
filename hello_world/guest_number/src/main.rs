use std::io;

fn main() {
    println!("猜数游戏");
    println!("请输入一个数字>>>");

    let mut guest = String::new();
    io::stdin().read_line(&mut guest)
        .expect("Failed to read line");

    println!("你输入的是: {}", guest)
}
