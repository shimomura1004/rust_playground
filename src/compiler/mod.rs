use interpreter::Ast;
use vm;

pub fn compile(ast : &Ast, code : &mut Vec<vm::Operator>) {
    match ast {
        Ast::Add(t1, t2) => {
            compile(t1, code);
            compile(t2, code);
            code.push(vm::Operator::Add);
        },
        Ast::Sub(t1, t2) => {
            compile(t1, code);
            compile(t2, code);
            code.push(vm::Operator::Sub);
        },
        Ast::Mul(t1, t2) => {
            compile(t1, code);
            compile(t2, code);
            code.push(vm::Operator::Mul);
        },
        Ast::Div(t1, t2) => {
            compile(t1, code);
            compile(t2, code);
            code.push(vm::Operator::Div);
        },
        Ast::App(t1, t2) => {
            // todo
        },
        Ast::Var(name) => (),
        Ast::Num(num) => code.push(vm::Operator::PushInt32(*num)),

        Ast::Assign(name, ast) => {
            
        }
    };
}
