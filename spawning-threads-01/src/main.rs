use std::thread;
use std::time::Duration;

fn main() {
    for i in 1..=5 {
        thread::spawn(move || {
            println!("Hello from thread # {}", i);
            thread::sleep(Duration::from_secs(3));
        });
    }
    thread::sleep(Duration::from_secs(1));
}
