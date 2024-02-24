use compiler::build_compiler;

fn main() {
    let code = "if (i < 10){ var j = 10; printl(j)}";
    let mut compiler = build_compiler(code);
    compiler.compile();
}