extern crate rand;

use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("please input guess");

        // mutableな変数
        let mut guess = String::new();

        // &mut guessは、上で宣言したguess変数を参照している
        io::stdin().read_line(&mut guess).expect("Failed to read line");

        // シャドーイングをしている。
        // すでに宣言された変数に、新しい値を覆い隠すことができる。
        // guessの型として、u32(非負整数)を指定することで、parseメソッドが文字列をどんな数字に変換するのか決める。
        // parseもResult型を返す
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("you guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("too small"),
            Ordering::Greater => println!("too big"),
            Ordering::Equal => {
                println!("you win");
                break;
            },
        }
    }

}
