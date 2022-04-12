use nats::{*, jetstream::Error};
// use nats::{jetstream::Error};

fn main() -> Result<(),Error> {

    println!("START!");

    let nc = nats::connect("localhost:4222")?;

    let nc2 = nats::Options::with_user_pass("derek", "s3cr3t!")
        .with_name("My Rust NATS App")
        .connect("127.0.0.1")?;

    println!("END!");

    Ok(())
    
}






