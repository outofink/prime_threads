use std::sync::mpsc;
use std::thread;

struct PrimeTask {
    start: u32,
    length: u32,
    tx: mpsc::Sender<u32>,
}
impl PrimeTask {
    fn count(&self) -> u32 {
        (self.start..self.start + self.length)
            .filter(|&x| is_prime(x))
            .count() as u32
    }
    fn find_primes(self) {
        thread::spawn(move || {
            self.transmit(self.count());
        });
    }
    fn new(start: u32, length: u32, tx: mpsc::Sender<u32>) -> PrimeTask {
        PrimeTask { start, length, tx }
    }
    fn transmit(&self, count: u32) {
        self.tx.send(count).expect("Sorry I failed");
    }
}
fn is_prime(n: u32) -> bool {
    if n == 2 || n == 3 {
        return true;
    }
    if n <= 1 || n % 2 == 0 || n % 3 == 0 {
        return false;
    }
    let mut i = 5;
    while i * i <= n {
        if n % i == 0 || n % (i + 2) == 0 {
            return false;
        }
        i += 6;
    }
    true
}

pub fn count_primes(numbers: u32, threads: u32, elements_per_task: u32) -> u32 {
    assert!(
        numbers % (threads * elements_per_task) == 0,
        "Number does not divide evenly over number of tasks."
    );
    assert!(
        numbers % elements_per_task == 0,
        "Number does not divide evenly over number of threads."
    );
    let tasks = numbers / elements_per_task;
    let mut total = 0;
    let mut receivers = Vec::with_capacity(threads as usize);

    for i in 0..threads {
        let (tx, rx) = mpsc::channel();
        receivers.push(Some(rx));
        PrimeTask::new(i * elements_per_task, elements_per_task, tx).find_primes();
    }
    let mut tasks_left = numbers / elements_per_task - threads;
    while receivers.iter().any(|r| r.is_some()) {
        for receiver in receivers.iter_mut() {
            if receiver.is_none() {
                continue;
            }
            if let Ok(count) = receiver.as_mut().unwrap().try_recv() {
                total += count;
                if tasks_left == 0 {
                    *receiver = None;
                    continue;
                }
                tasks_left -= 1;
                let (tx, rx) = mpsc::channel();
                *receiver = Some(rx);
                PrimeTask::new(
                    (tasks - tasks_left - 1) * elements_per_task,
                    elements_per_task,
                    tx,
                )
                .find_primes();
            }
        }
    }
    total
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::is_prime;

    #[test]
    fn test_is_prime() {
        assert_eq!(is_prime(1), false);
        assert_eq!(is_prime(2), true);
        assert_eq!(is_prime(3), true);
        assert_eq!(is_prime(4), false);
        assert_eq!(is_prime(5), true);
        assert_eq!(is_prime(15485863), true);
        assert_eq!(is_prime(15485861), false);
    }
    #[test]
    fn test_count_primes_1000() {
        assert_eq!(count_primes(1000, 1, 1000), 168);
        assert_eq!(count_primes(1000, 2, 500), 168);
        assert_eq!(count_primes(1000, 4, 250), 168);
        assert_eq!(count_primes(1000, 8, 125), 168);
    }
    #[test]
    fn test_count_primes_10000() {
        assert_eq!(count_primes(10000, 8, 1250), 1229);
        assert_eq!(count_primes(10000, 8, 125), 1229);
        assert_eq!(count_primes(10000, 8, 25), 1229);
        assert_eq!(count_primes(10000, 8, 5), 1229);
    }
}
