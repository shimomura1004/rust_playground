#[derive(Debug, Copy, Clone)]
pub enum Operator {
    PushInt32(i32),
    Pop,

    Add,
    Sub,
    Mul,
    Div,

    Not,

    Equal,             // read 2 value from stack, compare them  and push 1/0 if values are the same/different

    Load(usize),       // read the n-th item in the stack and push it on top
    Store(usize),      // write value on top of the stack to the n-th item in the stack

    Print,             // print the value on top of the stack

    JumpIf(isize),     // proceed the PC if top of the stack is not 0
    JumpUnless(isize), // proced the PC if top of the stack is 0
    Jump(isize),       // proceed the PC for the size

    Dump,
}

#[derive(Debug, Copy, Clone)]
enum Data {
    Num(i32),
}

pub fn process(program : &Vec<Operator>) {
    let mut pc : usize = 0;
    //let mut sp : usize = 0;
    let sp : usize = 0;
    let mut stack : Vec<Data> = Vec::new();

    while pc < program.len() {
        println!("{:?}", stack);
        match program[pc] {
            Operator::PushInt32(i) => stack.push(Data::Num(i)),
            Operator::Pop => {stack.pop();},

            Operator::Add => {
                let Data::Num(v1) = stack.pop().unwrap();
                let Data::Num(v2) = stack.pop().unwrap();
                stack.push(Data::Num(v2 + v1));
            },

            Operator::Sub => {
                let Data::Num(v1) = stack.pop().unwrap();
                let Data::Num(v2) = stack.pop().unwrap();
                stack.push(Data::Num(v2 - v1));
            },


            Operator::Not => {
                let Data::Num(n) = stack.pop().unwrap();
                if n == 0 {
                    stack.push(Data::Num(1));
                }
                else {
                    stack.push(Data::Num(0));
                }
            },

            Operator::Equal => {
                let Data::Num(v1) = stack.pop().unwrap();
                let Data::Num(v2) = stack.pop().unwrap();
                if v2 == v1 {
                    stack.push(Data::Num(1));
                }
                else {
                    stack.push(Data::Num(0));
                }
            },

            Operator::Load(n) => {
                let data = stack[sp + stack.len() - n - 1];
                stack.push(data);
            },

            Operator::Store(n) => {
                let target_index = sp + stack.len() - n - 1;
                let source_index = sp + stack.len() - 1;
                stack[target_index] = stack[source_index];
            },

            Operator::Print => {
                let Data::Num(v1) = stack.last().unwrap();
                println!("{}", v1);
            },

            Operator::JumpIf(i) => {
                let Data::Num(v) = stack.pop().unwrap();
                if v != 0 {
                    pc = ((pc as isize) + i - 1) as usize;
                }
            },
            Operator::JumpUnless(i) => {
                let Data::Num(v) = stack.pop().unwrap();
                if v == 0 {
                    pc = ((pc as isize) + i - 1) as usize;
                }
            },
            Operator::Jump(i) => {
                pc = ((pc as isize) + i - 1) as usize;
            },

            Operator::Dump => {
                println!("{:?}", stack);
            },

            _ => panic!("Unknown instruction"),
        }
        
        pc += 1;
    }

    println!("STACK: {:?}", stack);
}


#[test]
fn vm_test() {
    // calculate sum of 1..10
    let program = vec![
        Operator::PushInt32(10),  // max
        Operator::PushInt32(0),   // counter
        Operator::PushInt32(0),   // sum
        Operator::Load(1),
        Operator::PushInt32(1),
        Operator::Add,
        Operator::Store(2),       // increment 'counter'
        Operator::Pop,
        Operator::Load(1),
        Operator::Add,            // update 'sum'
        Operator::Load(1),
        Operator::Load(3),
        Operator::Equal,          // compare 'max' and 'sum'
        Operator::Not,
        Operator::Dump,
        Operator::JumpIf(-12),
        Operator::Print,
    ];
    process(&program);
}
