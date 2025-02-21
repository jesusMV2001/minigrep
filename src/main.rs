use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let arg1 = &args[1];
    let arg2 = &args[2];

    println!("Searching for {arg1}");
    println!("In file {arg2}");
}
