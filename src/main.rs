mod lex;
mod parse;

fn main() {
    let input = " (+ -33.3 (- (* 2 3 4) (/ 9 3))) ";
    let ast = parse::parse(input);
    println!("{:?}", ast);
}
