use num_cpus;
use getrandom::fill;
use bip39::Mnemonic;
use sp_core::{Pair as PairTrait, crypto::Ss58Codec, sr25519::Pair};
use std::{
    env,
    io::Write,
    sync::{Arc, atomic::{AtomicU64, Ordering}},
    time::Instant,
};
use tokio::sync::mpsc;
use tokio::task::spawn_blocking;

#[tokio::main(flavor = "multi_thread")]
async fn main() {
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
    let (tx, mut rx) = mpsc::unbounded_channel::<(String, String)>();
    let start = Instant::now();

    for _ in 0..cores {
        let prefix = prefix.clone();
        let counter = Arc::clone(&counter);
        let tx = tx.clone();

        spawn_blocking(move || {
            let mut seed = [0u8; 32];
            loop {
                counter.fetch_add(1, Ordering::Relaxed);

                fill(&mut seed).expect("RNG failure");
                let pair = Pair::from_seed(&seed);
                let addr = pair.public().to_ss58check();
                if addr.len() > SKIP && addr[SKIP..].starts_with(&prefix) {
                    let phrase = Mnemonic::from_entropy(&seed)
                        .unwrap()
                        .to_string();
                    let _ = tx.send((addr, phrase));
                    break;
                }
            }
        });
    }

    let mut interval = tokio::time::interval(std::time::Duration::from_secs(1));

    loop {
        tokio::select! {
            Some((addr, phrase)) = rx.recv() => {
                println!();
                let elapsed = start.elapsed();
                let tried  = counter.load(Ordering::Relaxed);
                println!("🎉 Found it in {:.2?} after {} tries!", elapsed, tried);
                println!("Address:  {}", addr);
                println!("Mnemonic: {}", phrase);
                std::process::exit(0);
            }

            _ = interval.tick() => {
                let elapsed = start.elapsed();
                let tried = counter.load(Ordering::Relaxed);
                let rate = tried as f64 / elapsed.as_secs_f64().max(1e-6);
                print!(
                    "\rTried {:>12} addresses | elapsed {:>6.2?} | {:>8.0} addr/sec",
                    tried, elapsed, rate
                );
                std::io::stdout().flush().unwrap();
            }
        }
    }
}
