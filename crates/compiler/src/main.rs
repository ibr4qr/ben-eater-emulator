use assembler::parser::build_parser;
use compiler::build_compiler;
use vm::emulator::build_emulator;

// THe compiling process will produce the ben eater machine code direclty using th Assembler crate
fn main() {
    let code = "var aNumber = 10 - 1; print(aNumber);";
    let mut compiler = build_compiler(code);

    // TODO: Do this only the compiler is running to run the program
    // and not only to compile
    compiler.compile();
    println!("{:?}", compiler.assembly_code);
    let mut assembler = build_parser();
    assembler.parse(compiler.assembly_code);

    let mut emulator = build_emulator();
    emulator.load_program(&assembler.binary_code);
    emulator.execute_program();
}
