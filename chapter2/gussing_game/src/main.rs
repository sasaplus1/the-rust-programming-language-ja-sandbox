// $ cargo doc --open で使用しているクレートのドキュメントを見ることができる

use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    // 1..101は1以上101未満の範囲を表す
    // 1..=100という範囲を渡すこともできる
    let secret_number = rand::thread_rng().gen_range(1..101);

    println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        // std::io::stdin()
        io::stdin()
            // &guessでなく&mut guessなので参照を可変とする
            .read_line(&mut guess)
            // io::Resultはexpectを持っておりプログラムをクラッシュできる
            .expect("Failed to read line");

        // 同じ名前で変数を宣言することでシャドーイングできる
        // parse()のために変数の型を明示的に指定する必要がある
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
