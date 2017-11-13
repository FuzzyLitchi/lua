mod lua;

use lua::lexer::lexer;

fn main() {
    let code = "
foo = 123
    ";

    let lexer = lexer(code.chars().collect());
    for token in lexer {
        println!("{:?}", token);
    }
}
