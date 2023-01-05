use std::{io, cmp::Ordering};
use rand::Rng;

fn main() {
    println!(r#"Adivinhe o número!"#);
    let secret_number = rand::thread_rng().gen_range(1..=100);
    loop {
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Falha ao ler a informação.");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,            
        };

        println!("Você escolheu o número: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Muito baixo!"),
            Ordering::Greater => println!("Muito grande!"),
            Ordering::Equal => {
                println!("Acertou!");
                break;
            }
        }
    }
}