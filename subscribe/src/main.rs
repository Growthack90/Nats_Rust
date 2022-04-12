use nats;
use std::io::Error;
use std::thread;
use std::time::Duration;

fn main() -> Result<(),Error> {

    println!("START!");

    // SUBSCRIBE
    let nc = nats::connect("localhost:4222")?;
    let sub = nc.subscribe("test")?;

/*         while x < 10 {
            for x in 0..10 {
                println!("Helloooo {}!!!", 100);
                thread::sleep(Duration::from_millis(1000)); // wait by 1 sec
                println!("{}", x + 1);
                if x == 10 {
                    break;
                }
            }
        } */    

        let mut x = 0;

        for msg in sub.messages() {
            thread::sleep(Duration::from_millis(1000)); // wait by 1 sec
            println!("Helloooo {}!!!", 100);
            x += 1;
            if x == 10 {
                break;
            }
        }

        println!("END!");

    Ok(())   
}





