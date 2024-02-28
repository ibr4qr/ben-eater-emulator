use crate::{ast_nodes::Node, memory_manager::{build_memory_manager, MemoryManager}, token::{Token, TokenType}};




pub struct CodeGen {
    ast: Vec<Node>,
    code: String,
    memory_manager: MemoryManager,
}


impl CodeGen {
    pub fn gen_code(&mut self) {
       for node in self.ast.clone() {
        let code = self.gen_node_code(node);
        println!("{}", code);
       }
    }

    pub fn gen_node_code(&mut self, node: Node) -> String {
        let mut code = String::new();
        match node {
            Node::BinaryExpr { operator, right, left } => {
                let mut left_code = self.gen_node_code(*left);
                let mut right_code = self.gen_node_code(*right);
                let mut binary_code = self.gen_binary_code(operator);

                let address = self.memory_manager.get_memory_offset();

                left_code =  "LDI ".to_string() + &left_code;
                left_code = left_code + "\nSTA " + &address.to_string(); 

                /*
                    get the address from memory manager,
                    this will be an offset, we then refine 
                    that address by knowing the program length, 
                    if the program length + the number of values stored inside the
                    rom is greater that 16 bytes than we need to throw
                    an error, as said, rom of 16 bytes is the biggest limitation
                    of the architecture we are trying to compile to...
                */
                
                right_code = "LDI ".to_string() + &right_code;
                binary_code = binary_code + " " + &address.to_string();
                code = left_code + "\n" + &right_code +  "\n" + &binary_code;
            },
            Node::Literal { value } => {
                code = code + &value.to_string();
            },
            Node::UnaryExpr { operator, right } => {
                self.gen_node_code(*right);
            },
            Node::PrintDecl { argument } => {
                 code = code + "\n" +  &self.gen_node_code(*argument) + "\n";
                 code = code + &self.gen_print_code();
            },
            Node::VarDecl { identifier, initializer } => {
              // produce code that will put its result into the A register
              // get memory address
              // store the content of A register into ROM
              code = self.gen_node_code(*initializer);
              code = code + "\n" + "STA";
            },
            Node::Nil => {

            },
            _ => println!("it s something else"),
        }
  
        return code;
    }




    pub fn gen_literal_code(&mut self, node: Node) {
        println!("")
    }

    pub fn gen_unary_code(&mut self, node: Node) {

    }

    pub fn gen_binary_code(&mut self, operator: Token) -> String {
       match operator.token_type {
        TokenType::Plus => {
            return "ADD".to_string();
        }
        TokenType::Minus => {
            return "SUB".to_string();
        },
        _ => {
            return "UNREACHABLE".to_string();
        }
       }
    }

    pub fn gen_print_code(&mut self) -> String {
       return "OUT".to_string();
    }
}



pub fn build_code_gen(ast: Vec<Node>) -> CodeGen {
    CodeGen {
        ast,
        code: String::new(),
        memory_manager: build_memory_manager()
    }
}