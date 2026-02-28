fn add_hungary(country_name: &mut String) {
  country_name.push_str("-Hungary");
  println!("{}", country_name);
}

fn main() {
  let mut country = String::from("Austria");
  add_hungary(&mut country);
  
  println!("{}", country);
}