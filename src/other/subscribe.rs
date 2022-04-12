let nc = nats::connect("demo.nats.io")?;
let sub = nc.subscribe("foo")?;
for msg in sub.messages() {
    println!("Helloooo {}!!!", 100)
}