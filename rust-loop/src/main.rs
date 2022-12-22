fn main() {
    // Print the first n positive integers
    let n = 10;
    let mut i = 1;
 
    loop {
        println!("loop: {}", i);  // statements inside the loop
        i = i + 1;                // increment
        if i > n {                // terminal condition
            break;
        }        
    }
 
    println!("in-between loops: {}", i);
    i = 1;
 
    while i <= n {                // terminal condition
        println!("while: {}", i); // statements inside the loop
        i = i + 1;                // increment
    }
 
    println!("in-between loops: {}", i);
 
    for i in 1..=10 {             // increment and terminal condition
        println!("for: {}", i);   // statements inside the loop
    }
 
    let cheat_code = [19, 65, 9, 17];
 
    for i in 0..cheat_code.len() {
        println!("cheat code: {}", cheat_code[i]);
    }
 
    for value in cheat_code.iter() {
        println!("cheat code by iter: {}", value);
    }

    for value in cheat_code {
        println!("cheat code by value: {}", value);
    }

    // 1. Convert temperatures between Fahrenheit and Celsius.
    let celsius1 = 37.0;
    let celsius2 = 0.0;
    let farenheit1 = 32.0;
    let farenheit2 = 104.0;
 
    println!("{} C = {:.1} F", celsius1, 32.0 + 9.0 / 5.0 * celsius1);
    println!("{} C = {} F", celsius2, 32.0 + 9.0 / 5.0 * celsius2);
    println!("{} F = {} C", farenheit1, (farenheit1 - 32.0) * 5.0 / 9.0);
    println!("{} F = {} C", farenheit2, (farenheit2 - 32.0) * 5.0 / 9.0);
    println!("\n\n");
    // 2. Generate the nth Fibonacci number.
    let n = 15;
    let mut previous = 0;
    let mut current = 1;
 
    for _ in 2..=n {
        let temp = previous + current;
        previous = current;
        current = temp;
    }
 
    println!("Fib({}) = {}\n\n", n, current);
 
    // 3. Print the lyrics to the Christmas carol
    // “The Twelve Days of Christmas,”
    // taking advantage of the repetition in the song.
    /*
    On the first day of Christmas
    My true love sent to me
    A partridge in a pear tree
    
    On the second day of Christmas
    My true love sent to me
    Two turtle-doves
    And a partridge in a pear tree
    
    On the third day of Christmas
    My true love sent to me
    Three French hens
    Two turtle-doves
    And a partridge in a pear tree
    
    On the fourth day of Christmas
    My true love sent to me
    Four calling birds
    Three French hens
    Two turtle-doves
    And a partridge in a pear tree    
    
    On the fifth day of Christmas
    My true love sent to me
    Five golden rings (five golden rings)
    Four calling birds
    Three French hens
    Two turtle-doves
    And a partridge in a pear tree
    
    On the sixth day of Christmas
    My true love sent to me
    Six geese a laying
    Five golden rings (five golden rings)
    Four calling birds
    Three French hens
    Two turtle-doves
    And a partridge in a pear tree
    */
    let first_line_prefix = "On the ";
    let first_line_suffix = " day of Christmas";
    let days = ["first", "second", "third", "fourth", "fifth", "sixth"];
    let things = [
        "And a partridge in a pear tree",
        "Two turtle-doves",
        "Three French hens",
        "Four calling birds",
        "Five golden rings (five golden rings)",
        "Six geese a laying"
    ];
 
    for i in 0..days.len() {
        println!(
            "{}{}{}\nMy true love sent to me",
            first_line_prefix,
            days[i],
            first_line_suffix);
        if i == 0 {
            println!("A partridge in a pear tree");
        } else {
            let mut j = i;
            loop {
                println!("{}", things[j]);
                if j == 0 {
                    break;
                }
                j = j - 1;
            }
        }
 
        println!();
    }


    
}