pub mod let nc = nats::connect("demo.nats.io")?;
nc.publish("my.subject", "Hello World!")?;