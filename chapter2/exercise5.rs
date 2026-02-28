// Can not return a ref from a function

fn return_str() -> String {
  let country = String::from("Austria");
  let country_ref = &country;
  country_ref.into()
}

fn main() {
  let country = return_str();
  println!("{}", country);
}