mod eval;
mod lex;
mod parse;

fn main() {
    let input = " (+ 5 (- (* 2 3 4) (/ 9 3))) ";
    match eval::evaluate(input) {
        Ok(num) => println!("{}", num),
        Err(_) => println!("Error"),
    };
}
