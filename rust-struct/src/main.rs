// ***Structs 
// Classes in Rust are called Structs. Structs are defined in two parts: 

// field specification: struct 
// methods: impl


// Fields of a class are defined in a struct
struct Candle {
    ticker: String,
    open: i64,
    high: i64,
    low: i64,
    close: i64,
    zeros: u32,
    year:u32,
    month:u32,
    day:u32,
}

// ***Traits 
// In other languages, we call Traits interfaces. 
// A trait (interface) is a promise that the struct (class) on which the trait is specified, 
// implements the trait. Without implementation, the code fails to compile.
trait Dateable {
    fn get_date(&self) -> String;
}

impl Dateable for Candle { 
    fn get_date(&self) -> String {
        format!("{}-{}-{}",self.year,self.month,self.day)
    }
}


// Methods of a class are defined using impl
impl Candle {
    // &self  - reference to this (see Javascript) or self (see python)
    fn convert_value_to_f64(&self, value: i64) -> f64 {
        value as f64 / (10_i64.pow(self.zeros)) as f64
    }
    fn get_delta(&self) -> f64 {
        self.convert_value_to_f64( self.close - self.open )
    }
    fn get_info(&self) -> String {
        format!(
        "{} ({}, {}, {}, {})",
        self.ticker,
        self.convert_value_to_f64(self.open),
        self.convert_value_to_f64(self.high),
        self.convert_value_to_f64(self.low),
        self.convert_value_to_f64(self.close)
        )
    }
}
fn main() {
    let today: Candle = Candle {
    ticker: "SOLUSDC".to_string(),
    open: 9522,
    high: 10883,
    low: 9517,
    close: 9981,
    zeros: 2,
    year:2022,
    month:3,
    day: 14
     };
    println!("{}", today.get_delta());
    println!("{}", today.get_info());
    println!("{}", today.get_date());
   
}