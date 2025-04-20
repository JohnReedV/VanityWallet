use sp_core::{Pair as PairTrait, crypto::Ss58Codec, sr25519::Pair};
use std::{
    env,
    io::Write,
    sync::{
        atomic::{AtomicU64, Ordering},
        Arc,
        mpsc,
    },
    thread,
    time::{Duration, Instant},
};

fn main() {
    let mut args = env::args();
    args.next();
    let prefix = args.next().unwrap_or_else(|| {
        eprintln!("Usage: vanity_wallet <prefix>");
        std::process::exit(1);
    });

    const SKIP: usize = 2;
    let cores = num_cpus::get();
    println!(
        "Searching on {} threads for SS58 addresses that starts with “{}”…",
        cores, prefix
    );

    let counter = Arc::new(AtomicU64::new(0));
    let (tx, rx) = mpsc::channel::<(String, String)>();

    for _ in 0..cores {
        let tx = tx.clone();
        let prefix = prefix.clone();
        let counter = Arc::clone(&counter);

        thread::spawn(move || {
            loop {
                counter.fetch_add(1, Ordering::Relaxed);
                let (pair, phrase, _seed) = Pair::generate_with_phrase(None);
                let addr = pair.public().to_ss58check();
                if addr.len() > SKIP && addr[SKIP..].starts_with(&prefix) {
                    let _ = tx.send((addr, phrase));
                    break;
                }
            }
        });
    }

    let start = Instant::now();
    loop {
        match rx.recv_timeout(Duration::from_secs(1)) {
            Ok((addr, phrase)) => {
                println!();
                let tried = counter.load(Ordering::Relaxed);
                let elapsed = start.elapsed();
                println!("🎉 Found it in {:.2?} after {} tries!", elapsed, tried);
                println!("Address:  {}", addr);
                println!("Mnemonic: {}", phrase);
                break;
            }
            Err(mpsc::RecvTimeoutError::Timeout) => {
                let tried = counter.load(Ordering::Relaxed);
                let elapsed = start.elapsed();
                let rate = tried as f64 / elapsed.as_secs_f64().max(1e-6);
                print!(
                    "\rTried {:>12} addrs | elapsed {:>6.2?} | {:>8.0} addr/sec",
                    tried, elapsed, rate
                );
                std::io::stdout().flush().unwrap();
            }
            Err(mpsc::RecvTimeoutError::Disconnected) => {
                eprintln!("\nAll threads exited without finding a match.");
                std::process::exit(1);
            }
        }
    }
}
