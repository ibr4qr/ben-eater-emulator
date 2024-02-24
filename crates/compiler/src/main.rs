use compiler::build_compiler;

fn main() {
    let code = "1+1";
    let mut compiler = build_compiler(code);
    compiler.compile();
}