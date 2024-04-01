pub mod repl;
pub mod token;
pub mod lexer;

fn main() {
	let input = "let add = fn(x, y) { x + y; };";

	let output = repl::new(input);
	println!("{:#?}", output);
}
