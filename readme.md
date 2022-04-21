# Nats_Rust

Progetto realizzato con il linguaggio RUST utilizzando il nats-server.

STEPS preliminari:

- installa Nats Server a questo [link](https://docs.nats.io/running-a-nats-service/introduction/installation);

- installa Rust a questo [link](https://doc.rust-lang.org/book/ch01-01-installation.html) oppure [qui](https://doc.rust-lang.org/cargo/getting-started/installation.html);

Per verificare che sono installati correttamente Rust - Cargo - Nats Server, eseguire da terminale:

- $ rustc --version

- $ cargo --version

- $ nats-server --version

Per l'esecuzione di questa app:

- $ cd ~/Nats_Rust

- Apri una finestra del terminale:
  $ nats-server -DV

- Apri un'altra finestra del terminale:
  $ cd publish/
  $ cargo run

- Apri un'altra finestra del terminale:
  $ cd subscribe/
  $ cargo run

Lets try it!

;-)
