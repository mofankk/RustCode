// use guess_game;

// 猜数字游戏，能够处理用户输入、错误处理、循环判断、退出循环

use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Hello, world!");
    // guess_game();
    ownership();
}

fn guess_game() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);
    // println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

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

fn ownership() {
    let s1 = String::from("hello");
    takes_ownership(s1);    // s1的值移动到函数里
    // println!("{}", s1);            // s1的值在这里不再有效

    let x = 5;
    makes_copy(5);  // x的值复制到函数里
    println!("{}", x);         // x在这里仍然有效

    let s2 = String::from("world");
    copy_ownership(s2.clone());     // 把s2复制一份到函数里
    println!("{}", s2);                       // 所以s2在这里仍然有效

    let (s3, length) = calculate_length(s2);
    println!("{},{}", s3, length);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn copy_ownership(some_string: String) {
    println!("{}", some_string);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();

    (s, length)
}
