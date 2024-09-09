// 定数は型が必要でどこでも宣言できる
const MAX_POINTS: u32 = 100_000;

/*
fn main() {
    // letだけだと変更できない
    // mutをつけると変更可能になる
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
}
*/

fn main() {
    println!("max points is: {}", MAX_POINTS);

    // シャドーイング
    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }
    println!("The value of x is: {}", x);

    let spaces = "   ";
    let spaces = spaces.len();

    // 以下は型が異なるのでコンパイルエラーになる
    // let mut spaces = "    ";
    // spaces = spaces.len();

    println!("spaces length is: {}", spaces);
}
