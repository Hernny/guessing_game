use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Adivina el numero");
    let secret_number: u32 = rand::thread_rng().gen_range(0..=100);

    loop {
        println!("Por favor introduzca un numero del 0 al 100");
        let mut guess: String = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Error de entrada, intente de nuevo introduciendo un numero valido");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("Introdujo el numero {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Mmmm! no, muy Bajo"),
            Ordering::Greater => println!("Por poco pero, pero te fuiste!"),
            Ordering::Equal => {
                println!("Yeeeiii! Ganaste!!!");
                break;
            }
        }
    }
}
