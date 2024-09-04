use std::io;

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new();

    // std::io::stdin()
    io::stdin()
        // &guessでなく&mut guessなので参照を可変とする
        .read_line(&mut guess)
        // io::Resultはexpectを持っておりプログラムをクラッシュできる
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
}
