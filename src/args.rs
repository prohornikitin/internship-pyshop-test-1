use clap::Parser;
use clap_num::number_range;

fn from_1_to_32(s: &str) -> Result<u8, String> {
    number_range(s, 1, 32)
}

#[derive(Parser)]
struct RawArgs {
    #[arg(
        short='N',
        default_value="1",
        value_parser=from_1_to_32,
        help="number of trailing zeros which hash should have"
    )]
    trailing_zeros: u8,

    #[arg(
        short='F',
        default_value="1",
        help="number of hashes to find"
    )]
    hashes_needed: usize,

    #[arg(
        short='j', 
        long="threads",
        help="number of threads [default: number of cpu logical cores]"
    )]
    threads: Option<usize>,

    #[arg(
        long="chunk-size",
        default_value="4096",
        help="how many numbers process on each thread for one job"
    )]
    chunk_size: usize,
}

pub struct Args {
    pub trailing_zeros: u8,
    pub hashes_needed: usize,
    pub threads: usize,
    pub chunk_size: usize,
}

impl Args {
    pub fn parse() -> Args {
        let raw = RawArgs::parse();
        return Args {
            trailing_zeros: raw.trailing_zeros,
            hashes_needed: raw.hashes_needed,
            threads: raw.threads.unwrap_or(num_cpus::get()),
            chunk_size: raw.chunk_size,
        }
    }
}
