use std::io;
use std::io::Write;
mod vm;
mod parser;
mod interpreter;
mod compiler;

fn main() {
    let mut interpreter = interpreter::Interpreter::new();
    let mut expression = String::new();

    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        expression.clear();
        io::stdin().read_line(&mut expression)
            .expect("Failed to read line");

        let parse_result = parser::Block::new().parse(&mut expression.trim().to_string());

        match parse_result {
            Ok(block) => {
                let ast = parser::syntax::block_to_ast(block);
                println!("AST: {:?}", ast);
                
                // evaluate
                let v = interpreter.eval(ast.clone());
                match v {
                    Some(v) => {
                        match v {
                            interpreter::Data::Num(num) => {
                                println!("EVALUATED: {}", num);
                            },
                            interpreter::Data::Fun(_, env, _) => {
                                println!("EVALUATED: <fun>");
                                println!("{:?}", env);
                            },
                        }
                    },
                    None => println!("error"),
                };

                // compile
                let mut code = vec![];
                compiler::compile_block(&ast, &mut code);
                //compiler::compile_statement(&ast, &mut code);
                println!("ASSEMBLED: {:?}", code);

                // code.push(vm::Operator::Print);
                // code.push(vm::Operator::Pop);
                // vm::process(&code);
            },
            Err(e) => println!("AST: {:?}", e),
        }
    }
}
