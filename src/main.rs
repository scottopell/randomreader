use std::fs::File;
use std::io::{self, Read};
use std::thread::sleep;
use std::time::Duration;

fn main() -> io::Result<()> {
    loop {
        // Read a few bytes from /dev/urandom
        let mut file = File::open("/dev/urandom")?;
        let mut buffer = [0u8; 16]; // Adjust the size as needed
        file.read_exact(&mut buffer)?;

        // Perform a simple computation: sum the bytes
        let sum: u16 = buffer.iter().map(|&b| b as u16).sum();
        println!("Sum of random bytes: {}", sum);

        // Sleep for a short duration
        sleep(Duration::from_millis(500)); // Adjust the sleep duration as needed
    }
}
