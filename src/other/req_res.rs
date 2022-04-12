let nc = nats::connect("demo.nats.io")?;
let resp = nc.request("foo", "Help me?")?;

// With a timeout.
let resp = nc.request_timeout("foo", "Help me?", Duration::from_secs(2))?;