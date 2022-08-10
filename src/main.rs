use std::io;
fn main() {

    let mut way = String::new();
    println!("Fahrenheit and Celsius temperature converter!");

    println!("Which way would you like to convert temperatures?
    Type '1' for Celsius to Fahrenheit.
    Type '2' for Fahrenheit to Celsius.");

    io::stdin()
    .read_line(&mut way)
    .expect("Failed to read line");

    let way: u8 = match way.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("Please input a number")
    };

    if way == 1 {
        c_f()
    } else if way == 2 {
        f_c()
    } else {
        println!("Please input '1' for C to F, or '2' for F to C.
        No other inputs are valid")
    };


}


fn c_f() {
    let mut c = String::new();
    let _f: f32;
    
    println!("What Celcius temperature would you like to convert to Fahrenheit?");

    io::stdin()
    .read_line(&mut c)
    .expect("Failed to read line");

    let c: f32 = match c.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("Please input a number")
    };

    println!("Calculating...");

    let f = c * 1.8 + 32.0;

    println!("{c}C is {f}F");

    
}


fn f_c() {
    let mut f = String::new();
    let _c: f32;
    
    println!("What Fahrenheit temperature would you like to convert to Celsius?");

    io::stdin()
    .read_line(&mut f)
    .expect("Failed to read line");

    let f: f32 = match f.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("Please input a number")
    };

    println!("Calculating...");

    let c = f - 32.0;
    let c = c * 0.55555555555;


    println!("{f}F is {c}C");

}