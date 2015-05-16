mod lex;

fn main() {
    let input = "  (  * + / - -33.3  )   ";
    let tokenizer = lex::Tokenizer::new(input);
    for token in tokenizer {
        println!("{:?}", token);
    }
}
