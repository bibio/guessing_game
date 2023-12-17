use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("数をあててごらん!");

    // 開始..終了で、終了は含まない
    let secret_number = rand::thread_rng().gen_range(1..101);

    loop {
        println!("ほら、予想を入力してね。");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("行の読み込みに失敗しました。");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("あなたの予想: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("小さすぎ!"),
            Ordering::Greater => println!("大きすぎ!"),
            Ordering::Equal => {
                println!("あたり!");
                break;
            }
        }
    }
}
