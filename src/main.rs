use std::{io::{self, Write, stdout, stdin}, process::exit};
use termion::raw::IntoRawMode;
use termion::input::TermRead;
use std::cmp::Ordering;
use rand::Rng;

fn pause() {
    let mut stdout = stdout().into_raw_mode().unwrap();
    write!(stdout, r#"Pressione qualquer tecla para continuar..."#).unwrap();
    stdout.flush().unwrap();
    stdin().keys().next();
}

fn win() {
    println!("Wow!!");
    exit(0);
}

fn cpu() {
    println!(r#"Adivinhe o número! (inteiro de 1 a 100)"#);
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

        clearscreen::clear().unwrap();
        println!("Você escolheu o número: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Muito baixo!"),
            Ordering::Greater => println!("Muito grande!"),
            Ordering::Equal => {
                println!("Acertou!");
                pause();
                clearscreen::clear().unwrap();
                break;
            }
        }
    }
}

fn human() {
    println!(r#"Guarde na sua memória um número inteiro de 1 a 100!"#);
    pause();
    let mut minimo = 1;
    let mut maximo = 100;
    let secret_number = rand::thread_rng().gen_range(minimo..=maximo);
    human_loop(secret_number, &mut minimo, &mut maximo);
}

fn human_loop(number: i32, min: &mut i32, max: &mut i32) {
    loop {
        println!("\r\n");
        println!("O número é {number}?\n");
        println!("1 - O número é MENOR;");
        println!("2 - O número é MAIOR;");
        println!("3 - Acertou!");
        
        let mut resp = String::new();

        io::stdin()
            .read_line(&mut resp)
            .expect("Falha ao ler a informação.");

        let resp: u32 = match resp.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,            
        };

        match resp {
            1 => less(number, min, max),
            2 => greater(number, min, max),
            3 => win(),
            _ => break,
        }
        
        fn less(number: i32, min: &mut i32, max: &mut i32) {
            *max = number - 1;
            let new_number = rand::thread_rng().gen_range(*min..=*max);
            human_loop(new_number, min, max);
        }

        fn greater(number: i32, min: &mut i32, max: &mut i32) {
            *min = number;
            let new_number = rand::thread_rng().gen_range(*min..=*max);
            human_loop(new_number, min, max);
        }
    }
}

fn main() {
    println!("Jogo de escolha de número!");
    loop {
        println!("Escolha o modo:\n");
        println!("1 - CPU escolhe e você adivinha;");
        println!("2 - Você escolhe e CPU adivinha;");
        println!("3 - Sair.");
        let mut choose:String = String::new();
        io::stdin()
                .read_line(&mut choose)
                .expect("Falha ao ler a informação.");
        let choose: u32 = match choose.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        clearscreen::clear().unwrap();
        match choose {
            1 => cpu(),
            2 => human(),
            3 => break,
            _ => continue,
        }
    }
}