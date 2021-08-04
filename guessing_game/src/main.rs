use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("숫자를 맞혀보아요~!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("사용자가 맞혀야 할 숫자 : {}", secret_number);

    println!("정답이라고 생각하는 숫자는~?");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("딥력한 값을 읽지 못했어요ㅠㅠ");

    let guess: u32 = guess.trim().parse()
        .expect("입력한 값이 올바른 숫자가 아닙니다.");

    println!("입력한 값은 {}", guess);

    match guess.cmp(&secret_number){
        Ordering::Less => println!("입력한 숙자가 작아요~"),
        Ordering::Greater => println!("입력한 숫자가 큽니다~"),
        Ordering::Equal => println!("정확해요~! 맞아요~! 야호~!")
    }

}
