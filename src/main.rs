use std::{ops::Range, thread::{self, JoinHandle}};

use regex::Regex;
use sha256;
use clap::Parser;

#[derive(Parser, Debug)]
// #[command(0.1, Command, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short='N', default_value="1", help="number of trailing zeros which hash should have")]
    trailing_zeros: u64,

    /// Number of times to greet
    #[arg(short='F', default_value="1", help="number of hashes to generated")]
    hashes_needed: u64,
}


fn has_exatly_n_trailing_zeros(n: u64) -> Regex {
    Regex::new(format!(r"0{{{}}}$", n).as_str()).unwrap()
}


#[derive(Clone, Debug)]
struct Variant {
    input: u64,
    digest: String,
}

fn spawn_worker_thread(inputs: Range<u64>, regex: Regex) -> JoinHandle<Vec<Variant>> {
    thread::spawn(move|| {
        let mut results: Vec<Variant> = vec![];
        for i in inputs {
            let digest = sha256::digest(i.to_string());
            if regex.is_match(digest.as_str()) {
                results.push(Variant {
                    input: i,
                    digest: digest,
                });
            }
        }
        return results;
    })
}

const THREADS: u64 = 4;
const CHUNK_SIZE: u64 = 1024*256;
fn main() {
    let args = Args::parse();

    let mut new_chunk_start = 1;
    let mut found = 0;
    let regex = has_exatly_n_trailing_zeros(args.trailing_zeros);
    while new_chunk_start < 18446744073709551615 && found < args.hashes_needed  {
        let mut handles: Vec<JoinHandle<Vec<Variant>>> = vec![];
        for _ in 1..THREADS {
            handles.push(spawn_worker_thread(
                new_chunk_start..new_chunk_start+CHUNK_SIZE,
                regex.clone()
            ));
            new_chunk_start += CHUNK_SIZE;
        }
        let mut results: Vec<Variant> = vec![];
        for h in handles {
            for v in h.join().unwrap() {
                results.push(v);
            }
        }
        results.sort_unstable_by_key(|x| x.input);
        for v in results {
            println!("{}, {}", v.input, v.digest);
            found += 1;
            if found >= args.hashes_needed {
                break;
            }
        }
        if found >= args.hashes_needed {
            break;
        }
    }
    
}
