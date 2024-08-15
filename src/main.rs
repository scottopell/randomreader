use clap::{ArgGroup, Parser};
use rand::seq::SliceRandom;
use rand::thread_rng;
use rand::Rng;
use std::fs::File;
use std::io::{self, Read};
use std::thread::sleep;
use std::time::Duration;
use std::time::Instant;

#[derive(Parser, Debug)]
#[clap(author, version, about)]
#[clap(group(
    ArgGroup::new("buffer_size")
        .required(false)
        .args(&["buffer_size_bytes", "buffer_size_kb", "buffer_size_mb"])
))]
struct Config {
    #[clap(long, env = "BUFFER_SIZE_BYTES")]
    buffer_size_bytes: Option<usize>,

    #[clap(long, env = "BUFFER_SIZE_KB")]
    buffer_size_kb: Option<usize>,

    #[clap(long, env = "BUFFER_SIZE_MB")]
    buffer_size_mb: Option<usize>,

    #[clap(long, default_value = "4096")]
    default_buffer_size: usize,
}

fn main() -> io::Result<()> {
    let config = Config::parse();
    let mut rng = thread_rng();

    let buffer_size_bytes = if let Some(bytes) = config.buffer_size_bytes {
        bytes
    } else if let Some(kb) = config.buffer_size_kb {
        kb * 1024
    } else if let Some(mb) = config.buffer_size_mb {
        mb * 1024 * 1024
    } else {
        config.default_buffer_size
    };

    let mut file = File::open("/dev/urandom")?;
    let mut current_buffer_size_bytes = buffer_size_bytes;

    loop {
        let mut buffer: Vec<u8> = vec![0; current_buffer_size_bytes];

        let now = Instant::now();
        file.read_exact(&mut buffer)?;
        println!(
            "Read random {current_buffer_size_bytes} bytes in {}ms, now shuffling...",
            now.elapsed().as_millis()
        );

        // Perform a simple computation: sum the bytes
        buffer.shuffle(&mut rng);
        let elapsed = now.elapsed();
        println!(
            "Shuffling complete, took {}ms, now sleeping...",
            elapsed.as_millis()
        );

        let adjustment = rng.gen_range(-0.5..=0.5);
        let oldbuf = current_buffer_size_bytes;
        current_buffer_size_bytes =
            buffer_size_bytes.saturating_sub((buffer_size_bytes as f64 * adjustment) as usize);
        sleep(Duration::from_millis(500));
    }
}
