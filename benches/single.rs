use std::time::Instant;

use cooking::{Cook, CookError};

fn main() -> Result<(), CookError> {
    let start = Instant::now();
    let cycles = 1_000_000;

    let cook = Cook::new()?;

    for _ in 0..cycles {
        cook.cook(&[
            "Yellow Lizalfos Tail"
        ])?;
    }

    let elapsed = start.elapsed();
    let cycles_per_second = cycles as f64 / elapsed.as_secs_f64();
    println!("Cooked {} meals in {:.2?} ({:.2} cycles/s)", cycles, elapsed, cycles_per_second);

    Ok(())
}