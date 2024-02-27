use crate::{ast_nodes::Node, token::{Token, TokenType}};




pub struct CodeGen {
    ast: Vec<Node>,
    code: String
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
                let left_code = self.gen_node_code(*left);
                let right_code = self.gen_node_code(*right);
                let binary_code = self.gen_binary_code(operator);

                code = left_code + "\n" + &right_code +  "\n" + &binary_code;
            },
            Node::Literal { value } => {
                code = code + "LDI ";
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
        code: String::new()
    }
}