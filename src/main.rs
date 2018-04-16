include std::io;

fn main() {
    println!("Guess the number!");
    println!("Please input your guess: ");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
      .expect("Failed to read user's input");

    println!("You guessed {}", guess);  

}
