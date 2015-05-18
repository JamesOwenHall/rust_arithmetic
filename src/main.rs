use std::env;

mod eval;
mod lex;
mod parse;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: ./arithmetic expression");
        return;
    }

    match eval::evaluate(&args[1]) {
        Ok(num) => println!("{}", num),
        Err(_) => println!("Error"),
    };
}
