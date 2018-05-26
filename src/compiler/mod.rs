use parser::syntax::ExpAst;
use vm;

pub fn compile(ast : &ExpAst, code : &mut Vec<vm::Operator>) {
    match ast {
        ExpAst::Add(t1, t2) => {
            compile(t1, code);
            compile(t2, code);
            code.push(vm::Operator::Add);
        },
        ExpAst::Sub(t1, t2) => {
            compile(t1, code);
            compile(t2, code);
            code.push(vm::Operator::Sub);
        },
        ExpAst::Mul(t1, t2) => {
            compile(t1, code);
            compile(t2, code);
            code.push(vm::Operator::Mul);
        },
        ExpAst::Div(t1, t2) => {
            compile(t1, code);
            compile(t2, code);
            code.push(vm::Operator::Div);
        },
        ExpAst::App(t1, t2) => {
            // todo
        },
        ExpAst::Var(name) => (),
        ExpAst::Num(num) => code.push(vm::Operator::PushInt32(*num)),
    };
}
