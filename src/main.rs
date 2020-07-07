use std::io;
use rand;
use std::cmp::Ordering;

use rand::Rng;

fn main() {
    println!("test");
    println!("숫자를 입력하세요");

    let mut s : bool = true;
    while(s){
        let mut guess = String::new();
        let rnd = rand::thread_rng().gen_range(1, 11);

        io::stdin().read_line(&mut guess)
        .expect("error1");

        let guess : i32 = guess.trim().parse().expect("숫자를 입력하세요.");

        println!("입력한 숫자 : {}, 랜덤한 숫자 : {}", guess, rnd);

        match guess.cmp(&rnd){
            Ordering::Less => println!("랜덤 수 보다 작습니다."),
            Ordering::Equal => s = false,
            Ordering::Greater => println!("랜덤 수 보다 큽니다."),

        }
    }
}
