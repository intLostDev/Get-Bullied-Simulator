use std::io;
fn main() {
    println!("How much money you got?!");

    let mut money = String::new();

    io::stdin()
        .read_line(&mut money);

    for _i in 0..1 {
        money.pop();
    }

    let moneyvalue: i32 = money.parse().unwrap();

    println!("Well I got {}", moneyvalue*2);

    let mut finalline = String::new();

    io::stdin()
        .read_line(&mut finalline);
}