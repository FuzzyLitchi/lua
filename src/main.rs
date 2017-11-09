mod lua;

use lua::lexer::lexer;
use lua::lexer::tokenizer::Tokenizer;

fn main() {
    let code = "1234";

    let mut lexer = lexer(code.chars().collect());
    println!("{:?}", lexer.match_token());
}
