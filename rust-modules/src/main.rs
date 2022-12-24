use rust_modules::dependency::mod_one::one as dependency;

fn main() {
    let secret1 = rust_modules::secret::f1();

    let secret2 = rust_modules::f3();
    println!("data: {} {}", secret1, secret2);
    println!("One from dependency: {}", dependency());
}

// cargo run --bin rust-modules
// cargo run --bin binary_crate