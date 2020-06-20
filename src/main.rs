use std::time::{Instant};
use std::env;

mod prime_finder;

fn main() {
  let args: Vec<String> = env::args().collect();

  match args.len() {
    1 => { println!("You must provide at least a positive integer."); return; }
    _ => {}
  }

  let upper_limit = match args[1].parse() {
    Ok(u) => u,
    Err(x) => {
      println!("Failed to parse parameter: {}", x);
      return;
    }
  };

  let start_time = Instant::now();
  let prime_list = prime_finder::compute_for(upper_limit);
  let time_spent = start_time.elapsed();

  println!("{:?}", prime_list);
  println!("Found in {:?}", time_spent);
}
