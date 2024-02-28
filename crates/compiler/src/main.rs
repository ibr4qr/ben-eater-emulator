use compiler::build_compiler;

fn main() {
    let code = "print(10+4); print(20 +2);";
    let mut compiler = build_compiler(code);
    compiler.compile();
}