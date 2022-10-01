use rand::Rng;
use std::cmp::Ordering;
use std::io;
fn main() {
    println!("猜数字的游戏");
    // 获取神秘数字
    let secrect_bumber = rand::thread_rng().gen_range(0..=100);

    // loop 循环
    loop {
        println!("guess a number");

        // 获取标准输入的数据
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("无法读取数据 ");

        // 将获取的string数据转化为u32类型
        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("your guess number is {}", guess);

        // 比较数据的大小
        match guess.cmp(&secrect_bumber) {
            Ordering::Less => println!("too small guess number"),
            Ordering::Greater => println!("too big guess number"),
            Ordering::Equal => {
                println!("you win!!!!");
                break;
            }
        }
    }
}
