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
        io::stdout().flush();
        expression.clear();
        io::stdin().read_line(&mut expression)
            .expect("Failed to read line");

        let parse_result = parser::Statement::new().parse(&mut expression.trim().to_string());

        match parse_result {
            Ok(statement) => {
                let ast = parser::syntax::statement_to_ast(statement);
                
                // evaluate
                let v = interpreter.eval(ast.clone());
                match v {
                    Some(v) => {
                        match v {
                            interpreter::Data::Num(num) => {
                                println!("{}", num);
                            },
                            interpreter::Data::Fun(_, _) => {
                                println!("<fun>");
                            },
                        }
                    },
                    None => println!("error"),
                };

                // compile
                let mut code = vec![];
                compiler::compile_statement(&ast, &mut code);
                println!("ASSEMBLED: {:?}", code);

                code.push(vm::Operator::Print);
                code.push(vm::Operator::Pop);
                vm::process(&code);
            },
            Err(e) => println!("AST: {:?}", e),
        }
    }
}
