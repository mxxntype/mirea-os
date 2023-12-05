use std::thread;
use std::time::Duration;

fn main() {
    let thread_handle = thread::spawn(|| {
        for i in 1..=5 {
            println!("\nПоток выполняется {}с...", i);
            thread::sleep(Duration::from_secs(1));
        }
    });
    thread_handle.join().unwrap();
    thread::sleep(Duration::from_secs(10));
}
