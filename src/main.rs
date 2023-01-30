use prime_threads::*;
use std::{thread::available_parallelism, time::Instant};
// use std::time::Instant;

// const NUMBERS: i32 = 1_000_000;
// const ELEMENTS_PER_THREAD: i32 = 1000;
// const TASKS: i32 = NUMBERS / ELEMENTS_PER_THREAD;

fn pp(i: u32) -> String {
    let mut s = String::new();
    let i_str = i.to_string();
    let a = i_str.chars().rev().enumerate();
    for (idx, val) in a {
        if idx != 0 && idx % 3 == 0 {
            s.insert(0, ',');
        }
        s.insert(0, val);
    }
    s
}
#[allow(dead_code)]
fn seive(until: usize) -> usize {
    let mut sieve = vec![true; until as usize];
    sieve[0] = false;
    sieve[1] = false;
    for i in 2..until {
        if sieve[i as usize] {
            let mut j = i * i;
            while j < until {
                sieve[j] = false;
                j += i;
            }
        }
    }
    sieve.iter().filter(|&x| *x).count()
}

#[allow(dead_code)]
fn mod_seive(max: usize) -> usize {
    let mut sieve = vec![true; max];
    let mut prime = vec![];
    prime.reserve(max/2);
    let mut spf = vec![0; max];
    sieve[0] = false;
    sieve[1] = false;
    for i in 2..max {
        if sieve[i] {
            prime.push(i);
            spf[i] = i;
        }
        for j in 0..prime.len() {
            if prime[j] > spf[i] || i * prime[j] >= max {
                break;
            }
            sieve[i * prime[j]] = false;
            spf[i * prime[j]] = prime[j];
        }
    }
    sieve.iter().filter(|&x| *x).count()
}

fn main() {
    let threads = available_parallelism().unwrap().get() as u32;
    let counts = [1000, 10_000, 100_000, 1_000_000, 10_000_000, 100_000_000];
    for num in counts {
        for elem in counts {
            if elem > num {
                continue;
            }
            let start = Instant::now();
            let count = count_primes(num, threads, elem / threads);
            let duration = start.elapsed();
            println!(
                "Threads: {0} | Numbers: {1:<12}| Elements per task: {2:<11}| Count: {3:<10}| Duration: {4:.2?}",
                threads, pp(num), pp(elem / threads), pp(count), duration
            );
        }
    }
    // let start = Instant::now();
    // println!("{:?}", seive(1_000_000_000));
    // let duration = start.elapsed();
    // println!("Duration (seive): {0:.2?}", duration);
    // let start = Instant::now();
    // println!("{:?}", mod_seive(1_000_000_000));
    // let duration = start.elapsed();
    // println!("Duration (log(n)): {0:.2?}", duration);
    // let start = Instant::now();
    // println!("{:?}", segmented_sieve(1_000_000_000));
    // let duration = start.elapsed();
    // println!("Duration (segmented): {0:.2?}", duration);
}
