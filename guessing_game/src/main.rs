use std::io;

fn main() {
    println!("숫자를 맞혀보아요~!");

    println!("정답이라고 생각하는 숫자는~?");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("딥력한 값을 읽지 못했어요ㅠㅠ");

    println!("입력한 값은? : {}", guess);

}
