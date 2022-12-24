//Create a new lib:
//cargo new --lib roman

//Run the test:
//cargo test 


pub fn convert(roman_numeral: &str) -> u32 {
    // For now, we are not making use of roman_numeral, so we place an 
    // underscore in front of it to indicate we expect the compiler not to warn us 
    // about the unused variable.

    let mut sum = 0;
    for ch in roman_numeral.chars() {
        println!("ch {}", ch);
        match ch {
            'I' => { sum += 1; } 
            'V' => { sum += 5; }
            'X' => { sum += 10; } 
            'L' => { sum += 50; }
            'C' => { sum += 100; } 
            'D' => { sum += 500; } 
            'M' => { sum += 1000; } 
            _ => {}
        }
    }

    return sum;
}

#[cfg(test)]
mod tests {
    // To test this convert function, we have to understand that tests is also a 
    // module. To import the convert function, we have to use the super 
    // keyword inside the tests module to reference the parent. If we want to 
    // import everything, we can simply use the asterisk wild card:
    use super::*;

    //set up
    #[test]
    fn it_should_return_one() {
        assert_eq!(convert("I"), 1);
    }

    #[test]
    fn it_should_return_two() {
        assert_eq!(convert("II"), 2);
    }

    #[test]
    fn it_should_handle_additive_digits() {
        assert_eq!(convert("VII"), 7);
        assert_eq!(convert("MMMDCCCLXXXVIII"), 3888);
    }

    //tear down

}
// Use test driven development: 
// 1.Write a failing test
// 2.Write just enough code to make it pass
// 3.Refactor – simplify the code without changing its behavior. This step is guarded by your tests


