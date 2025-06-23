pub mod lexer;
pub mod repl;
pub mod token;

fn main() {
    let input = "let add = fn(x, y) { x + y; };";

    let output = repl::new(input);
    println!("{:#?}", output);
}
