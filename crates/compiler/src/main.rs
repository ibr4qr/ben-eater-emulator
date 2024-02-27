use compiler::build_compiler;

fn main() {
    let code = "print(10); print(13 +2); var a = 10+20; print(100);";
    let mut compiler = build_compiler(code);
    compiler.compile();
}