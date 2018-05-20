mod vm;
use vm::Operator::*;
mod parser;
use parser::Parser;

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
    let p_1 = parser::ParserChar {c : '1'};
    let result = p_1.parse(code);
    println!("{:?}", result);

    let code = "11123";
    let p_ones = parser::ParserMany1 {p : &p_1};
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

    let code = "23";
    let p_2 = parser::ParserChar {c: '2'};
    let p_try = parser::ParserTry {ps: vec![&p_1, &p_2]};
    let one_or_two = p_try.parse(code);
    println!("{:?}", one_or_two);
}
