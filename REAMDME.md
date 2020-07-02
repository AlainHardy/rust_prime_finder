# Prime Finder
This program was made to get an idea of how Rust works.

It uses threads to find the list of prime numbers from 1 to a given argument.

## How to use ?
It take at least a positive integer value as an argument.
There are two optional arguments:
- **-t** or **--threads**   : Choose the number of system threads that Rust must spawn.
- **-s** or **--splits**    : Number of subdivision to make in the search range. Threads compute prime numbers from those subdivisions. When the number of splits is greater than the number of threads, threads will continue to work until all subdivisions have been processed.

Example for using 2 threads and creating 4 subdivisions for the range from 1 to 100 :
```sh
./rust_prime_finder -t 2 -s 4 100
```
