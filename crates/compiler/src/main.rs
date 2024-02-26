use compiler::build_compiler;

fn main() {
    let code = "print(10+20); var me = 100;";
    let mut compiler = build_compiler(code);
    compiler.compile();
}