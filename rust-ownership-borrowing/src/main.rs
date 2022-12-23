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

    // *********************************************************************
    // ***Functions and ownership
    // function arguments have the scope of a function 
    // when returning a value in a function, the returned value’s ownership is transferred to 
    // the variable holding the return value of the function.
    let str: String = "reference".to_string();
    let str2: String = f(str);

    //println!("{}",str); // Error : borrow of moved value
    println!("{}",str2);


    ///////////////////////////////////////////////////////
    // What happened here?
    // We create a mutable String with value “reference”
    // We pass this string to a function called double_str 
    // Upon passing “reference” to double_str, ownership of reference is transferred to argument s of double_str. The scope of s is within the function body. In main, str gets invalidated as it lost ownership of “reference” 
    // A new string “referencereference” is created and returned by double_str 
    // Upon exiting double_str, s gets out of scope, and therefore, the memory storing “reference” is freed 
    // Upon returning “referencereference” the new owner of this new string is str2 
    // Consequence: as str is invalidated, we cannot use its value in main

    let str3: String = "reference".to_string();
    let str4: String = double_str(str3);

    //println!("{}",str3); // Error : borrow of moved value
    println!("{}",str4);


    // *********************************************************************
    // ***Borrowing
    // We might not want ownership transfer to happen, as in double_str, we are only using the value of s without owning it:
    // The & operator only takes the reference of str.
    // when &str is passed to double_str, the value “reference” is still owned by str 
    // s is just a reference, therefore, once s gets out of scope, we do not need to free 
    // anything in memory, as s does not own anything
    // The act of taking the reference of a value without having ownership of it is called borrowing. Once the borrowed reference gets out of scope, the owner still owns its value.
    let str5: String = "reference".to_string();
    let str6: String = double_str_borrowing(&str5);

    println!("{}",str5); 
    println!("{}",str6);

}

fn f(s:String) ->String {   // scope of s within these braces
    let new_str: String = s; // new_str now owns "reference"
    return new_str;          // ownership is transferred to the Value of
                             // Value = f(str)
}

fn double_str(s:String) -> String {
    return format!("{}{}",s,s)
}

fn double_str_borrowing(s:&String) -> String {
    return format!("{}{}",s,s)
}




