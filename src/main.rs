mod lua;

use lua::lexer::lexer;

fn main() {
    let code = "
foo = 123
hahaja = \"haha\"
bar = true
    ";

    let lexer = lexer(code.chars().collect());
    for token in lexer {
        println!("{:?}", token);
    }
}
