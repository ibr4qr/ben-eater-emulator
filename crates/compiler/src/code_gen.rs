use crate::{
    ast_nodes::Node,
    memory_manager::{build_memory_manager, MemoryManager},
    token::{Token, TokenType},
};

pub struct CodeGen {
    ast: Vec<Node>,
    memory_manager: MemoryManager,
}

impl CodeGen {
    pub fn gen_code(&mut self) -> Vec<String> {
        let mut assembly_code_instructions: Vec<String> = Vec::new();
        for node in self.ast.clone() {
            let code = self.gen_node_code(node);
            assembly_code_instructions.push(code);
        }

        let instructions: Vec<String> = assembly_code_instructions
            .iter()
            .flat_map(|block| block.split('\n'))
            .filter(|s| !s.is_empty())
            .map(|s| s.to_string())
            .collect();

        return instructions;
    }

    #[allow(unused_variables)]
    #[allow(unreachable_patterns)]
    pub fn gen_node_code(&mut self, node: Node) -> String {
        let mut code = String::new();
        match node {
            Node::BinaryExpr {
                operator,
                right,
                left,
            } => {
                let left_expression_result_address = self.memory_manager.get_memory_offset();

                // the left expression will save the result in A register
                // we need to save it in a empty memory position
                // so we can use it later when adding it to the right
                // expression result

                code += &self.gen_node_code(*right);
                code += &format!("STA {}\n", left_expression_result_address);

                code += &self.gen_node_code(*left);
                // the right expression will save the result in A register

                let binary_code = self.gen_binary_code(operator);

                code += &format!("{} {}\n", binary_code, left_expression_result_address);
                // free the space
                self.memory_manager
                    .free_memory_offset(left_expression_result_address);
            }
            Node::Literal { value } => {
                code += &format!("LDI {}\n", value);
            }
            Node::UnaryExpr { operator, right } => {
                self.gen_node_code(*right);
            }
            Node::PrintDecl { argument } => {
                code += &self.gen_node_code(*argument);
                code += "OUT\n";
            }
            Node::VarDecl {
                identifier,
                initializer,
            } => {
                // produce code that will put its result into the A register
                // get memory address
                // store the content of A register into ROM

                let address = self.memory_manager.get_memory_offset();

                // here the generated initialization code will put the result of
                // the expression in A register
                code += &self.gen_node_code(*initializer);

                code += &format!("STA {}\n", address);
            }
            Node::Nil => {}
            _ => println!("it s something else"),
        }

        return code;
    }

    #[allow(unused_variables)]
    pub fn gen_literal_code(&mut self, node: Node) {
        println!("")
    }
    #[allow(unused_variables)]
    pub fn gen_unary_code(&mut self, node: Node) {}

    pub fn gen_binary_code(&mut self, operator: Token) -> String {
        match operator.token_type {
            TokenType::Plus => {
                return "ADD".to_string();
            }
            TokenType::Minus => {
                return "SUB".to_string();
            }
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
        memory_manager: build_memory_manager(),
    }
}
