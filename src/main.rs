use std::env;
use terbilang;

fn get_number_arg(args: &mut env::Args) -> Result<i64, String> {
    let mut args = args.take(2);
    let prog_name = args.next().unwrap();
    match args.next() {
        Some(arg) => match arg.parse::<i64>() {
            Ok(number) => Ok(number),
            Err(x) => Err(format!("{} `{}`", x, arg)),
        },
        _ => Err(format!("usage: {} <number>", prog_name)),
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let number: i64 = get_number_arg(&mut env::args())?;
    let t = terbilang::terbilang(number);
    println!("number: {}, terbilang: {}", number, t);
    Ok(())
}
