use std::io;
use std::io::Write;
mod vm;
use vm::Operator::*;
mod parser;
use parser::combinator::*;
mod interpreter;
mod compiler;

type Fun = Box<fn(Hoge) -> Hoge>;
enum Hoge<'a> {
    Int(i32),
    Fun(&'a Fun),
}

fn test<'a>(k : &'a str, v : &'a Hoge, aaa : &mut std::collections::HashMap<&'a str, &'a Hoge<'a>>) {
    let t : &'a Fun = &Box::new(|x| x);
    aaa.insert("ten", &Hoge::Int(10));
    aaa.insert("fun", &Hoge::Fun(t));
    aaa.insert(k, v);
    aaa.get("ten");
}

fn main() {
    let mut aaa = std::collections::HashMap::new();
    test("five", &Hoge::Int(5), &mut aaa);
    match aaa.get("fun") {
        Some(hoge) => {
            match hoge {
                Hoge::Int(i) => println!("{}", i),
                Hoge::Fun(f) => println!("function"),
            }
        },
        None => println!("Not found"),
    }

    let mut env = std::collections::HashMap::new();

    loop {
        let mut expression = String::new();

        print!("> ");
        io::stdout().flush();
        io::stdin().read_line(&mut expression)
            .expect("Failed to read line");

        let parse_result = parser::Statement{}.parse(&expression.trim());
        // println!("ParseResult: {:?}", parse_result);

        match parse_result {
            Ok((statement, _)) => {
                let ast = parser::syntax::statement_to_ast(statement);
                // println!("AST {:?}", ast);

                let v = interpreter::eval_ast(&ast, &mut env);
                match v {
                    Some(interpreter::Data::Num(num)) => println!("{}", num),
                    Some(interpreter::Data::Fun(_)) => println!("<fun>"),
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
fn vm_test() {
    // calculate sum of 1..10
    let program = vec![
        PushInt32(10),  // max
        PushInt32(0),   // counter
        PushInt32(0),   // sum
        Load(1),
        PushInt32(1),
        Add,
        Store(2),       // increment 'counter'
        Pop,
        Load(1),
        Add,            // update 'sum'
        Load(1),
        Load(3),
        Equal,          // compare 'max' and 'sum'
        Not,
        Dump,
        JumpIf(-12),
        Print,
    ];
    vm::process(&program);
}

#[test]
fn char_parser() {
    let code = "123";
    let p_one = Char {c : '1'};
    let result = p_one.parse(code);
    assert!(result.is_ok(), "parse error");
    let (c, _) = result.unwrap();
    assert_eq!(c, '1');
}

#[test]
fn many1_parser() {
    let code = "11123";
    let p_one = Char {c : '1'};
    let p_ones = Many1 {p : &p_one};
    let ones = p_ones.parse(code);
    assert!(ones.is_ok(), "parse error");
    let (ones, _) = ones.unwrap();
    assert_eq!(ones, vec!['1','1','1']);
}

#[test]
fn try_parser() {
    let code = "23";
    let p_try = Try {ps: vec![Box::new(Char {c: '1'}), Box::new(Char {c: '2'})]};
    let one_or_two = p_try.parse(code);
    assert!(one_or_two.is_ok(), "parse error");
    let (one_or_two, _) = one_or_two.unwrap();
    assert_eq!(one_or_two, '2');
}

#[test]
fn oneof_parser() {
    let code = "23";
    let p_one_or_two = OneOf::new("12");
    let one_or_two = p_one_or_two.parse(code);
    assert!(one_or_two.is_ok(), "parse error");
    let (one_or_two, _) = one_or_two.unwrap();
    assert_eq!(one_or_two, '2');
}

#[test]
fn digit_parser() {
    let code = "456a12";
    let i = Digit{}.parse(code);
    assert!(i.is_ok(), "parse error");
    let (i, _) = i.unwrap();
    assert_eq!(i, 456);
}
