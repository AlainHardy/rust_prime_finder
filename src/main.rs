use std::time::{Instant};
use std::thread;
use std::env;
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};

mod prime_finder;

fn main() {
  let args: Vec<String> = env::args().collect();

  let mut number_of_cpu = 1;
  let mut number_of_splits = 1;

  let mut counter = 1;

  while counter < args.len() {
    match args[counter].as_str() {
      "-t" | "--threads" => {
        counter += 1;
        number_of_cpu = match if counter >= args.len() {
          println!("Missing number of cpu argument to use.");
          return;
        } else { args[counter].parse() } {
          Ok(u) => u,
          Err(_x) => {
            println!("Failed to parse the number of cpu to use.");
            return;
          }
        };
      }
      "-s" | "--splits" => {
        counter += 1;
        number_of_splits = match if counter >= args.len() {
          // Divide the range of search in multiple smaller ranges.
          println!("Missing number for a division in smaller ranges of the original range.");
          return;
        } else { args[counter].parse() } {
          Ok(u) => u,
          Err(_x) => {
            println!("Failed to parse the number of splits to make.");
            return;
          }
        }
      }
      _ => break
    }
    counter += 1;
  }

  // Check if there still is a parameter left for the value used to search primes.
  match args.len() {
    length if counter >= length => { println!("You must provide at least a positive integer."); return; }
    _ => {}
  }

  let upper_limit:u32 = match args[counter].parse() {
    Ok(u) => u,
    Err(x) => {
      println!("Failed to parse parameter: {}", x);
      return;
    }
  };

  let step = (upper_limit as f32 / (number_of_splits as f32)).ceil() as usize;
  let portions: Arc<Mutex<VecDeque<_>>> = Arc::new(Mutex::new((1..upper_limit).step_by(step).collect::<VecDeque<_>>()));

  let mut threads: Vec<thread::JoinHandle<_>> = Vec::with_capacity(number_of_cpu);

  let start_time = Instant::now();
  let result_aggregate: Arc<Mutex<Vec<Vec<u32>>>> = Arc::new(Mutex::new(vec![Vec::new(); number_of_splits as usize]));

  for _ in 0..number_of_cpu {
    let p = Arc::clone(&portions);
    let ag = Arc::clone(&result_aggregate);
    threads.push(thread::spawn(move || {
      let mut result: Vec<(u32, Vec<u32>)> = Vec::new();
      // Fetch portions of numbers, and compute primes for this portion.
      // If no portions are left, leave the computing process.
      loop {
        let mut vec_mutex = p.lock().unwrap();
        let value = match vec_mutex.pop_front() {
          Some(x) => x,
          None => { break; }
        };
        std::mem::drop(vec_mutex);

        result.push((value, prime_finder::compute_from_to(value,
          if value + step as u32 > upper_limit { upper_limit } else { value + step as u32 }
        )));
      }

      // Write the results back in a list that contains all primes found by the differents threads.
      let mut global_result = ag.lock().unwrap();
      for prime_result in result {
        let index = ((prime_result.0 - 1) / step as u32) as usize;
        global_result[index] = prime_result.1;
      }
    }));
  }

  for thread in threads {
    let _ = thread.join();
  }

  let time_spent = start_time.elapsed();

  let result_clone = Arc::clone(&result_aggregate);
  let result_content = result_clone.lock().unwrap();
  let prime_list: Vec<&u32> = result_content.iter().flatten().collect();

  println!("{:?}", prime_list);
  println!("Found in {:?}", time_spent);
}
