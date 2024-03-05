use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let msg = String::from("Hello from the sender!");
        tx.send(msg).unwrap();
    });
    let received = rx.recv().unwrap();
    println!("Received: {}", received);
}
