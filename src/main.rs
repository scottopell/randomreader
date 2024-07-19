use clap::{ArgGroup, Parser};
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::fs::File;
use std::io::{self, Read};
use std::thread::sleep;
use std::time::Instant;

#[derive(Parser, Debug)]
#[clap(author, version, about)]
#[clap(group(
    ArgGroup::new("buffer_size")
        .required(true)
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

    let buffer_size_bytes = if let Some(bytes) = config.buffer_size_bytes {
        bytes
    } else if let Some(kb) = config.buffer_size_kb {
        kb * 1024
    } else if let Some(mb) = config.buffer_size_mb {
        mb * 1024 * 1024
    } else {
        config.default_buffer_size
    };

    let mut buffer = vec![0u8; buffer_size_bytes];
    let mut file = File::open("/dev/urandom")?;

    loop {
        let now = Instant::now();
        file.read_exact(&mut buffer)?;
        println!(
            "Read random bytes in {}ms, now shuffling...",
            now.elapsed().as_millis()
        );

        // Perform a simple computation: sum the bytes
        buffer.shuffle(&mut thread_rng());
        let elapsed = now.elapsed();
        println!(
            "Shuffling complete, took {}ms, now sleeping...",
            elapsed.as_millis()
        );

        sleep(elapsed);
    }
}
