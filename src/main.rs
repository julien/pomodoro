use std::thread;
use std::time::{Duration, Instant};

fn main() {
    let total = Duration::new(25 * 60, 0);
    let start = Instant::now();

    let handle = thread::spawn(move || 'inner: loop {
        let elapsed = start.elapsed();

        if elapsed >= total {
            break 'inner;
        }

        let remaining: u32 = match (total - elapsed).as_secs().try_into() {
            Ok(v) => v,
            Err(_) => {
                break 'inner;
            }
        };

        let hours = remaining / 60 / 60;
        let minutes = remaining / 60 - (hours * 60);
        let seconds = remaining - minutes * 60 - hours * 60 * 60;

        println!(
            "\x1B[2J\x1B[1;1HTime remaing: {:0>2}:{:0>2}",
            minutes, seconds
        );

        thread::sleep(Duration::from_secs(1));
    });

    handle.join().unwrap();
    println!("Take a break");
}
