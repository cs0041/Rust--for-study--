fn main() {
    // ***Ownership rules 
    // Owner of a value: block-scoped variable 
    // During execution, once a variable gets out of scope, its value is dropped 
    // A value can be owned by one variable, not more

    // ***Value and reference types
    // In many programming languages, we have value and reference types. Value types: 

    // Numeric values (signed, unsigned integer, float), 
    // booleans, 
    // characters, 
    // tuples containing only value types in any depth recursively. 
    // These types are said to implement the Copy trait. The Copy trait ensures that the value of these variables are copied upon assignment to a new memory location on the stack.

    // When a variable of these types is used in the right value, their value is substituted and the left value of the expression is given a new memory address (num2 contains the value 5 at a different memory address below):

    let mut num1: i32 = 5;
    let num2 : i32 = num1; // copies the current value of num1

    num1 +=1 ;
    println!("{} {}", num1, num2);

    // Once we assign a string to a new string variable, the original string variable gets 
    // invalidated. Therefore, we cannot even use str to print the value it holds. This is needed so
    // that once str gets out of scope, we don’t free memory referenced by str, as new_str now 
    // owns that area in the memory.
    let str: String = "reference".to_string();
    let new_str: String = str;

    // println!("{}",str); // Error : borrow of moved value
    println!("{}",new_str);

    //If you want to copy strings by value, clone them. Cloning is more time consuming though:
    let str2: String = "reference2".to_string();
    let new_str2: String = str2.clone();

    println!("{}",str2);
    println!("{}",new_str2);

}
