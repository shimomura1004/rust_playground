use std::io;
use std::io::Write;
use std::collections::HashMap;
mod vm;
mod parser;
use parser::combinator::*;
mod interpreter;
// mod compiler;

fn main() {
    //let mut env = HashMap::new();
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

                //let v = interpreter::eval_statement_ast(ast, &mut interpreter);
                let v = interpreter.eval(&ast);
                match v {
                    Some(v) => {
                        match v {
                            interpreter::Data::Num(num) => {
                                println!("{}", num);
                            },
                            interpreter::Data::Fun(_) => {
                                println!("<fun>");
                            },
                        }
                    },
                    None => println!("error"),
                };

                // let mut code = vec![];
                // compiler::compile(&ast, &mut code);
                // code.push(vm::Operator::Print);
                // println!("{:?}", code);

                // vm::process(&code);
            },
            Err(e) => println!("AST: {:?}", e),
        }
    }
}

#[test]
fn char_parser() {
    let mut code = "123".to_string();
    let p_one = Char::new('1');
    let result = p_one.parse(&mut code);
    assert!(result.is_ok(), "parse error");
    assert_eq!(result.unwrap(), '1');
}

#[test]
fn many1_parser() {
    let mut code = "11123".to_string();
    let p_one = Char::new('1');
    let p_ones = Many1::new(p_one);
    let ones = p_ones.parse(&mut code);
    assert!(ones.is_ok(), "parse error");
    assert_eq!(ones.unwrap(), vec!['1','1','1']);
}

#[test]
fn try_parser() {
    let mut code = "23".to_string();
    let p_try = Try::new(vec![Char::new('1'), Char::new('2')]);
    let one_or_two = p_try.parse(&mut code);
    assert!(one_or_two.is_ok(), "parse error");
    assert_eq!(one_or_two.unwrap(), '2');
}

#[test]
fn oneof_parser() {
    let mut code = "23".to_string();
    let p_one_or_two = OneOf::new("12");
    let one_or_two = p_one_or_two.parse(&mut code);
    assert!(one_or_two.is_ok(), "parse error");
    assert_eq!(one_or_two.unwrap(), '2');
}

#[test]
fn digit_parser() {
    let mut code = "456a12".to_string();
    let i = Digit::new().parse(&mut code);
    assert!(i.is_ok(), "parse error");
    assert_eq!(i.unwrap(), 456);
}

#[test]
fn combination_parser() {
    let whitespaces = SkipMany::new(Char::new(' '));
    let many_a = Many::new(Char::new('a'));
    let many_b = Many1::new(Char::new('b'));
    let many_c = Many1::new(Char::new('c'));
    let b_or_c = Try::new(vec![many_b, many_c]);
    
    let mut input = "aaabd".to_string();
    whitespaces.parse(&mut input);
    let parse_result = many_a.parse(&mut input);
    assert!(parse_result.is_ok(), "parse error");
    let parse_result = b_or_c.parse(&mut input);
    assert!(parse_result.is_ok(), "parse error");

    let mut input = "cd".to_string();
    whitespaces.parse(&mut input);
    let parse_result = many_a.parse(&mut input);
    assert!(parse_result.is_ok(), "parse error");
    let parse_result = b_or_c.parse(&mut input);
    assert!(parse_result.is_ok(), "parse error");
}

// #[test]
// fn vm_test() {
//     // calculate sum of 1..10
//     let program = vec![
//         PushInt32(10),  // max
//         PushInt32(0),   // counter
//         PushInt32(0),   // sum
//         Load(1),
//         PushInt32(1),
//         Add,
//         Store(2),       // increment 'counter'
//         Pop,
//         Load(1),
//         Add,            // update 'sum'
//         Load(1),
//         Load(3),
//         Equal,          // compare 'max' and 'sum'
//         Not,
//         Dump,
//         JumpIf(-12),
//         Print,
//     ];
//     vm::process(&program);
// }
