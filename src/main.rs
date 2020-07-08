use std::env;
use terbilang;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("rust-terbilang <number>");
        return;
    }
    let number_arg= &args[1];
    let number: terbilang::Number = number_arg.parse().expect("need a valid integer number");
    let t = terbilang::terbilang(&number).unwrap_or(String::from(""));
    println!("number: {}, terbilang: {}", number, t);
}
