use clap::Parser;

#[derive(Parser, Debug)]
pub struct Args {
    #[arg(
        short='N',
        default_value="1",
        help="number of trailing zeros which hash should have"
    )]
    pub trailing_zeros: usize,

    #[arg(
        short='F',
        default_value="1",
        help="number of hashes to generated"
    )]
    pub hashes_needed: usize,

    #[arg(
        short='j', 
        long="threads",
        default_value="4",
        help="number of threads"
    )]
    pub threads: usize,

    #[arg(
        long="chunk-size",
        default_value="4096",
        help="how many numbers process on each thread for one job"
    )]
    pub chunk_size: usize,
}
