use std::cmp as cmp;
// use rust_modules::dependency::mod_one::one as dependency;
use rust_modules::secret as secret;

fn main() {
    println!("Binary crate: {}", rust_modules::secret::f1());
    println!("Dependency from binary crate: {}", secret::f1());
    println!(
        "One from dependency: {}", rust_modules::dependency::mod_one::one());
    
    println!("{:?} {:?} {:?}", cmp::Ordering::Greater, cmp::Ordering::Less, cmp::Ordering::Equal);
}