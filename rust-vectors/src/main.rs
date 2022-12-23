fn main() {
    // Vector type
    // Rust’s arrays have a fixed number of elements.
    // The vector type behaves like a dynamic array, where you can add and remove elements.
    // Vector is a generic type. To create a vector, we need to specify the type of its elements. 
    // The elements of a vector have to have the same type.
    // Vec<type> is a vector with fixed type
    // The vec! macro creates a vector with given elements.
    
    // Normal
    let exponents: Vec<u32> = vec![1,2,8,16];
    for exp in exponents {
        println!("2.pow({}) = {}",exp,2_i32.pow(exp))
    }

    // Using referencing and dereferencing: 
    // Notice the &. Here we borrow the exponents vector.
    let exponents: Vec<u32> = vec![1,2,8,16];
    for exp in &exponents {
        println!("2.pow({}) = {}",exp,2_i32.pow(*exp))
    }
}
