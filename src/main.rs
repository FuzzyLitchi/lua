mod lua;

use lua::lexer::tokenizer::Tokenizer;

fn main() {
    let code = "haha yes";

    Tokenizer::new(code.chars().collect());
}
