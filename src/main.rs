use std::io;
use nats::*;

fn main() {

    let mut nome = String::new();
    println!("Enter your name: ");
    let n_caratteri = io::stdin().read_line(&mut nome).unwrap();
    println!("Hello , {}", nome);
    println!("numero caratteri nome: {}", n_caratteri);
    
    println!("***************************************");

}