fn main() {
  let mut my_number = 8;
  let num_ref  = &mut my_number;

  *num_ref += 10;
  println!("{}", my_number);
}