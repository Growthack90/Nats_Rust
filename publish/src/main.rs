use nats;
use std::io::Error;
use std::thread;
use std::time::Duration;

fn main() -> Result<(),Error> {

    println!("START!");

    // PUBLISH
    let nc = nats::connect("localhost:4222")?;
    println!("I am PUBLISH - Read me!");

    for x in 0..10 {
        thread::sleep(Duration::from_millis(1000)); // wait by 1 sec
        println!("{}", x + 1);
        nc.publish("test", "Hello World!")?;
    }

    println!("END!");

    Ok(())
    
}






