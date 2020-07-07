use std::io;

fn main() {
    println!("test");
    println!("숫자를 입력하세요");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
    .expect("error1");

    println!("입력한 숫자 : {}", guess);
}
