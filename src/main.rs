use std::time::{Instant};

mod prime_finder;

fn main() {
  let upper_limit = 100;
  let start_time = Instant::now();
  let prime_list = prime_finder::compute_for(upper_limit);
  let time_spent = start_time.elapsed();

  println!("{:?}", prime_list);
  println!("Found in {:?}", time_spent);
}
