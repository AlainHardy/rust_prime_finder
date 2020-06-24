pub fn compute_for(value: u32) -> Vec<u32> {
  compute_from_to(1, value + 1)
}

pub fn compute_from_to(down_limit: u32, upper_limit: u32) -> Vec<u32> {
  let mut ret: Vec<u32> = Vec::new();
  for number in down_limit..upper_limit {
    if is_prime(number) {
      ret.push(number);
    }
  }
  ret
}

fn is_prime(value: u32) -> bool {
  match value {
    1 => return false,
    2 => return true,
    _ => {
      for j in 2..=(value as f32).sqrt().ceil() as u32 { 
        if value % j == 0 { return false }
      }
    }
  }
  true
}
