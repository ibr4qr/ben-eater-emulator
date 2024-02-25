use compiler::build_compiler;

fn main() {
    let code = "100+100";
    let mut compiler = build_compiler(code);
    compiler.compile();
}