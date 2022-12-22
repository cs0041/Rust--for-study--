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
    // Constants are not declared using let 
    // type has to be declared 
    // the value of a constant cannot be computed during runtime
    // mut cannot be used with const
    // naming conventions: ALL_CAPS_WITH_UNDERSCORE
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
}
