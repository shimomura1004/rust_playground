use std::io;
use std::io::Write;
mod vm;
use vm::Operator::*;
mod parser;
use parser::combinator::*;
mod interpreter;

fn main() {
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

    let code = "123";
    let p_1 = Char {c : '1'};
    let result = p_1.parse(code);
    println!("{:?}", result);

    let code = "11123";
    let p_ones = Many1 {p : &p_1};
    let ones = p_ones.parse(code);
    println!("{:?}", ones);
    match ones {
        Ok((_, code)) => {
            let ones = p_ones.parse(code);
            println!("{:?}", ones);

            let one = p_1.parse(code);
            println!("{:?}", one);
        },
        Err(_) => (),
    }

    // let code = "23";
    // let p_2 = Char {c: '2'};
    // let p_try = Try {ps: &vec![&p_1, &p_2]};
    // let one_or_two = p_try.parse(code);
    // println!("{:?}", one_or_two);

    let code = "23";
    // let mut ps : Vec<Box<Parser<char>>> = vec![];
    // ps.push(Box::new(Char{c: '1'}));
    // ps.push(Box::new(Char{c: '2'}));
    // let p_try = Try{ps: ps};
    // let p_1 = Char {c: '1'};
    // let p_2 = Char {c: '2'};
    // let p_try = Try {ps: vec![Box::new(p_1), Box::new(p_2)]};
    // let one_or_two = p_try.parse(code);
    let p_one_or_two = OneOf{cs: vec!['1', '2']};
    let p_one_or_two = OneOf::new("12");
    let one_or_two = p_one_or_two.parse(code);
    println!("{:?}", one_or_two);

    let code = "456a12";
    let i = Digit{}.parse(code);
    println!("{:?}", i);

    loop {
        let mut expression = String::new();

        print!("> ");
        io::stdout().flush();
        io::stdin().read_line(&mut expression)
            .expect("Failed to read line");

        let parse_result = parser::Expression{}.parse(&expression.trim());
        println!("ParseResult: {:?}", parse_result);

        match parse_result {
            Ok((exp, _)) => {
                let ast = interpreter::exp_to_ast(exp);
                println!("AST {:?}", ast);

                let v = interpreter::eval_ast(ast);
                println!("Val: {}", v);
            },
            Err(e) => println!("AST: {:?}", e),
        }
        
    }
}
