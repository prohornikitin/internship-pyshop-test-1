mod args;
use std::{ops::Range, sync::mpsc::{self, Sender}};

use args::Args;
use clap::Parser;
use regex::Regex;
use sha256;
use threadpool::ThreadPool;


fn has_exatly_n_trailing_zeros(n: usize) -> Regex {
    Regex::new(format!(r"0{{{}}}$", n).as_str()).unwrap()
}


#[derive(Clone, Debug)]
struct HashCase {
    input: usize,
    digest: String,
}


fn find_cases_by_digest_regex(
    inputs: Range<usize>,
    needed_digest: Regex
) -> Vec<HashCase> {
    let mut results: Vec<HashCase> = Vec::new(); 
    for i in inputs {
        let digest = sha256::digest(i.to_string());
        if !needed_digest.is_match(digest.as_str()) {
            continue;
        }
        results.push(HashCase {
            input: i,
            digest: digest,
        });
    }
    return results;
}



struct TaskChunkInput {
    inputs: Range<usize>,
    needed_digest: Regex
}

impl TaskChunkInput {
    fn new(inputs: Range<usize>, needed_digest: &Regex) -> TaskChunkInput{
        TaskChunkInput {
            inputs,
            needed_digest: needed_digest.clone(),
        }
    }
}

fn schedule_task_chunk(
    pool: &ThreadPool,
    output: &Sender<Vec<HashCase>>,
    task: TaskChunkInput,
) {
    let output_clone = output.clone();
    pool.execute(move|| {
        output_clone.send(
            find_cases_by_digest_regex(task.inputs, task.needed_digest)
        ).unwrap();
    });
}

struct ChunksBoundsIterator {
    chunk_size: usize,
    next_chunk_start: usize,
}

impl ChunksBoundsIterator {
    fn new(chunk_size: usize, start: usize) -> ChunksBoundsIterator {
        return ChunksBoundsIterator {
            chunk_size,
            next_chunk_start: start,
        }
    }
}

impl Iterator for ChunksBoundsIterator {
    type Item = Range<usize>;

    fn next(&mut self) -> Option<Self::Item> {
        let start = self.next_chunk_start;
        let next_start_or_overflow = start.checked_add(self.chunk_size);
        if next_start_or_overflow.is_none() {
            return None;
        }
        let next_start = next_start_or_overflow.unwrap(); 
        self.next_chunk_start = next_start;
        return Some(start..next_start);
    }
}


fn main() {
    let args = Args::parse();
    let pool = ThreadPool::new(args.threads);
    let (sender, receiver) = mpsc::channel::<Vec<HashCase>>();

    let regex = has_exatly_n_trailing_zeros(args.trailing_zeros);
    let mut task_inputs = ChunksBoundsIterator::new(args.chunk_size, 1)
        .map(|range| TaskChunkInput::new(range, &regex));
    
    for _ in 0..pool.max_count() {
        let task = task_inputs.next().unwrap();
        schedule_task_chunk(&pool, &sender, task);    
    }
    
    let mut found = 0;
    let mut results: Vec<HashCase> = Vec::with_capacity(args.hashes_needed);

    while let Ok(received) = receiver.recv() {
        for case in received {
            results.push(case);
            found += 1;
        }

        if found >= args.hashes_needed {
            break;
        }
        let input = task_inputs.next();
        if input.is_none() {
            break;
        }
        schedule_task_chunk(&pool, &sender, input.unwrap())
    }

    drop(sender);
    pool.join();

    while let Ok(received) = receiver.recv() {
        for case in received {
            results.push(case);
            found += 1;
        }
    }


    results.sort_unstable_by_key(|x| x.input);

    for r in results.iter().take(args.hashes_needed) {
        println!("{}, {}", r.input, r.digest);
    }
}
