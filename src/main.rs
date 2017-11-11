mod lua;

use lua::lexer::lexer;

fn main() {
    let code = "1234
haha

haha
    ";

    let lexer = lexer(code.chars().collect());
    for token in lexer {
        println!("{:?}", token);
    }
}
