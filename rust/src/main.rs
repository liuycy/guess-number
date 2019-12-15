use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let correct = rand::thread_rng().gen_range(1, 101);
    println!("猜数字 (1 - 100) - 你猜是多少: ");

    loop {
        let mut answer = String::new();
        io::stdin().read_line(&mut answer).expect("读取失败 !");

        let answer: u32 = match answer.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("请输入数字: ");
                continue;
            }
        };

        match answer.cmp(&correct) {
            Ordering::Less => println!("小了, 再猜: "),
            Ordering::Greater => println!("大了, 再猜: "),
            Ordering::Equal => {
                println!("你猜对了 !");
                break;
            }
        }
    }
}
