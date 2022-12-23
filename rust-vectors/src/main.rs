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

    // What if we wanted to change the value of exponents? 
    let mut values:Vec<u32> = vec![1,2,8,16];
    for exp in &mut values{
        let value = 2_u32.pow(*exp);
        println!("2.pow({}) = {}",exp,value);
        *exp = value; // dereferencing: 
    }
    println!("{:?}",values);

    // ********************************************** 
    // It is also possible to build a vector from the ground up as mutable: 
    let mut n: Vec<u32> = Vec::new();

    n.push(1);
    n.push(2);
    n.push(8);
    n.push(16);

    for e in &n {
        println!("2 ^ {} : {}",e,2_i32.pow(*e));
    }

    // Iteration on indices using a while loop, accessing values of a Vector and change  the value of Vector:
    // !Dangerous it can index out of bounds: the program crashes.
    let len = n.len();
    let mut i = 0;
    while i < len  {
        n[i] -= 1;
        println!("{}",n[i]);
        i +=1 ;
        
    }
    println!("{:?}",n);

    // Reading Vector values in a safe way: 
    let mut i = 0;
    while i < 10  {
        match n.get(i) {
            Some(correct_value) => {
            println!("i: {} - n.get(i) : {}",i,correct_value);
           }
           None => {
            println!("index out of bounds - not exist");
           }
       }
       i +=1;
        
    }
    println!("{:?}",n);

    // Remove and pop elements from a vector: 

    // !Remove dangerous it can index out of bounds: the program crashes.
    let mut values: Vec<u32> = vec![1,2,8,16];
    let element1 = values.remove(0);
    println!("Removed element : {:?}",element1);
    println!("Vector : {:?}",values);

    // Pop safe than remove cause it no error  if index out of bounds just return None.
    let element2 = values.pop();
    println!("Popped element : {:?}",element2);
    println!("Vector : {:?}",values);
    
    // Using pop in a safe way: 
    let mut exponents: Vec<i32> = Vec::new();

    //exponents.push(1);
    //exponents.push(2);

    let pop_value: i32;
    match exponents.pop() {
        Some(value) => { pop_value = value; 
        }
        None => {
            pop_value = 0;
        }    
    }
    println!("{}",pop_value)

}
