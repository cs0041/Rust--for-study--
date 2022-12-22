fn main() {
    // Mutable and Immutable
    let mut x = 2;
    let y = 5;
    x = x+5;
    println!("Hello world! {} + {} = {}",x,y,x+y);

    // Unsigned integer
    let a: u8 = 255;
    let b: u16 = 65535;
    let c: u32 = 4294967295;
    let d: u64 = 18446744073709551615; 
    
    println!("{}, {}, {}, {}", a, b, c, d);

    let a_min: u8 = std::u8::MIN;
    let b_min: u16 = std::u16::MIN;
    let c_min: u32 = std::u32::MIN;
    let d_min: u64 = std::u64::MIN; 
    let e_min: usize = std::usize::MIN;

    let a_max: u8 = std::u8::MAX;
    let b_max: u16 = std::u16::MAX;
    let c_max: u32 = std::u32::MAX;
    let d_max: u64 = std::u64::MAX; 
    let e_max: usize = std::usize::MAX;
  
    println!("{}, {}, {}, {}, {}", a_min, b_min, c_min, d_min, e_min);
    println!("{}, {}, {}, {}, {}", a_max, b_max, c_max, d_max, e_max);

    // Signed integer
    let a_min: i8 = std::i8::MIN;
    let b_min: i16 = std::i16::MIN;
    let c_min: i32 = std::i32::MIN;
    let d_min: i64 = std::i64::MIN; 
    let e_min: isize = std::isize::MIN;

    let a_max: i8 = std::i8::MAX;
    let b_max: i16 = std::i16::MAX;
    let c_max: i32 = std::i32::MAX;
    let d_max: i64 = std::i64::MAX; 
    let e_max: isize = std::isize::MAX;
  
    println!("{}, {}, {}, {}, {}", a_min, b_min, c_min, d_min, e_min);
    println!("{}, {}, {}, {}, {}", a_max, b_max, c_max, d_max, e_max);

    // Floating point
    let a_min: f32 = std::f32::MIN;
    let b_min: f64 = std::f64::MIN; 

    let a_max: f32 = std::f32::MAX;
    let b_max: f64 = std::f64::MAX; 
  
    println!("{}, {}", a_min, b_min);
    println!("{}, {}", a_max, b_max);

    let a: f64 = 1.0; 
    let b: f64 = 0.1;
    let c: f64 = 0.2;
  
    println!("{}, {}", a, b + c);

    // Characters
    let ch1 = "X";
    let ch2 = "\u{2603}";
  
    println!("{}, {}", ch1, ch2);

    // Booleans
    let on: bool = true;
    let off: bool = false; 
  
    println!("{}, {}", on, off);
     
    // Constants 
    //*************************************
    // Constants are not declared using let 
    // type has to be declared 
    // the value of a constant cannot be computed during runtime
    // mut cannot be used with const
    // naming conventions: ALL_CAPS_WITH_UNDERSCORE
    //*************************************
    const PI:f64 = 3.1415;
    println!("{}",PI*2.0);

    // Operations
    // Must same type
    let first:f64 = 7.0;
    let second:f64 = 4.0;
 
    println!(
        "{} {} {} {} {}",
        first+second,
        first-second,
        first*second,
        first/second,
        first%second
    );

    //printIn and Format Macros
    let btc = "Bitcoin";
    let eth = "Ethereum";
    let sol = "Solana";
 
    println!("Normal order: {} {} {}", btc, eth, sol);
    println!("Flippening: {1} {0} {2} {2}", btc, eth, sol);
    println!(
        "Kwargs: {solana} {ethereum} {bitcoin}",
        bitcoin=btc,
        ethereum=eth,
        solana=sol
    );
 
    let message = format!(        
        "Kwargs: {solana} {ethereum} {bitcoin}",
        bitcoin=btc,
        ethereum=eth,
        solana=sol
    );
    println!("Look I've made this special {message}!", message=message);

    //Tuples
      let tuple = (1, 1.0, '1', true);
    println!("{} {} {} {}", tuple.0, tuple.1, tuple.2, tuple.3);
 
    let o = 135.1;
    let h = 139.5;
    let l = 133.7;
    let c = 133.8;
    let v = 100478;
    let token = ("Solana", "SOL", (o, h, l, c, v));
 
    println!(
        "{} ohlcv: ({}, {}, {}, {}, {})",
        token.0,
        (token.2).0,
        (token.2).1,
        (token.2).2,
        (token.2).3,
        (token.2).4
    );  
   
    let (name, ticker, ohlcv) = token;
    let (sol_o, sol_h, sol_l, sol_c, sol_v) = ohlcv;
 
    println!(
        "{} {} ohlcv: ({}, {}, {}, {}, {})",
        name,
        ticker,
        sol_o,
        sol_h,
        sol_l,
        sol_c,
        sol_v
    );
 
 
 
    println!(
        "{} ohlcv: {:?}",
        token.0,
        token.2
    );
 
    println!(
        "{} ohlcv: {:#?}",
        token.0,
        token.2
    );

    // Arrays 
    //*************************************
    // Rust arrays have a fixed length. 
    // Each element in a Rust array has the same type.
    // Essentially, arrays are like tuples, where the type of each element is fixed.
    //*************************************
    let cheat_code: [u32; 4] = [19, 65, 9, 17];
    let zeros = [1.2; 10];
 
    println!("Array: {:?}", cheat_code);
    println!("First element of the array: {}", cheat_code[0]);
 
    println!("Array(length: {}): {:?}", zeros.len() ,zeros);
 
    let slice = &cheat_code[1..3];
    println!("Slice of cheat_code: {:?} {}", slice, slice.len());

    //Strings
    //*************************************
    // 1.&str: string slice (referencing the variable type) / string literal (referencing the value encapsulated by double quotes). 
    // This is a (mostly) immutable character sequence.
    // may be stored on Stack, Heap, or may be embedded in the code 
    // optimized for runtime speed

    // 2.String: flexible, mutable string stored on the heap.

    // Converting a string literal to a mutable string: 
    // to_string method 
    // String::from method

    // Converting a mutable string to a string literal: 
    // & dereferencing in front of the string variable.
    //  It doesn’t copy the value, but just references the characters (see borrowing).
    //*************************************

    // &str: string slice
    // "": string literal
    let name: &str = "Zsolt";

    // String: mutable string
    let location: String = "Sliema, Malta".to_string();
    let title: String = String::from("IT Engineer");
 
    let title_slice = &title;
    println!("Hi, my name is {}.", name);
    println!("I live in {}.", location);
    println!("This name is {} characters long.", name.len());
    println!("My title is {}.", title);
    println!("Title as a string slice: {}.", title_slice);

    //*************************************
    // Getting a single character from a string: not that easy, because you have to check if the character exists.

    // String slicing: [Start…EndPlusOne], like in Python, the second index is not included.

    // Important: if you index out from the string, your program will crash 
    // If you want to get a slice until the end of the string, use [Start..] without specifying the end.
    //*************************************
    let digits = "0123456789".to_string();
 
    // Creating string slices
    let octal_digits = &digits[0..8];
    let binary_digits = &octal_digits[0..2];
 
    println!("{} {} {}", digits, octal_digits, binary_digits);
 
    // 'c' - character (primitive type)
    // "c" - immutable string literal
    // Referencing a single character from a string is harder
    if let Some(four) = &digits.chars().nth(12) {
        println!("Four: {}", four);
    } else {
        println!("Error");
    }   

    //*************************************
    // String concatenation: 
    // + only works if the left operand is a String and the right operand is an &str.
    // + is a left-associative operator, String + &str; → String.
    // format! macro, works like print! or println!, but instead of writing to the console, it returns a value 
    // array concatenation of &str values 
    // creating an empty string and pushing &str values to it
    //*************************************

    println!(
    "String concatenation: {} {} {}",
    "first".to_string() + "second",
    format!("{}{}", "first", "second"),
    ["first", "second"].concat(),
    );  
 
    let first = "1".to_string();
    let second = "2".to_string();
    let third = "3".to_string();
    let fourth = "4".to_string();
 
    println!(
        "String concatenation: {}",
        first + &second + &third + &fourth
    );

}
