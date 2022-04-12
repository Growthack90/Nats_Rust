use nats;
use std::io::Error;

fn main() -> Result<(),Error> {

    println!("START!");

    // SUBSCRIBE
    let nc = nats::connect("localhost:4222")?;
    let sub = nc.subscribe("test")?;
    for msg in sub.messages() {
        println!("Helloooo {}!!!", 100);
     
    print_message();

    }

    println!("END!");

    Ok(())
    
}

fn print_message(){
    println!("Hello, Domenico!");
}





