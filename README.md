# prime_threads

An implementation for finding the first n prime numbers using multiple threads,
in Rust.

This currently works by batching numbers into tasks and provides each thread a new
task when it's done the previous one such that all cores are saturated at any
given time.

Note: at the moment it uses a very naive algorithm of finding primes. The [Sieve
of Eratosthenes](https://en.wikipedia.org/wiki/Sieve_of_Eratosthenes) algorithm
currently runs 10x faster in a single thread that the current algorithm on 8
cores/16 threads.

This project was created to be a learning exercise in async Rust.
