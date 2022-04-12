use nats;
use std::io::Error;

fn main() -> Result<(),Error> {

    println!("START!");

    // PUBLISH
    let nc = nats::connect("localhost:4222")?;
    println!("I am PUBLISH - Read me!");
    nc.publish("test", "Hello World!")?;

    let mut check =  0;
    while check < 11{
    println!("Check is : {check}");
    check+=1;
    println!("After incrementing: {check}");

    if check == 10{
        break; // stop while
    }
    }

    println!("END!");

    Ok(())
    
}






