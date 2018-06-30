use parser::syntax::ExpAst;
use parser::syntax::StatementAst;
use parser::syntax::BlockAst;
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
        ExpAst::Var(name) => {
            // todo
        },
        ExpAst::Num(num) => code.push(vm::Operator::PushInt32(*num)),
        ExpAst::Fun(_,_) => {
            // todo
        },
        ExpAst::If(cond_exp, then_exp, else_exp) => {
            compile(cond_exp, code);
            code.push(vm::Operator::PushInt32(0));
            code.push(vm::Operator::Equal);

            let mut then_code = vec![];
            compile(then_exp, &mut then_code);
            let then_size = then_code.len();

            code.push(vm::Operator::JumpIf(then_size as isize + 2));
            code.append(&mut then_code);

            let mut else_code = vec![];
            compile(else_exp, &mut else_code);
            let else_size = else_code.len();

            code.push(vm::Operator::Jump(else_size as isize + 1));
            code.append(&mut else_code);
        },
    };
}

pub fn compile_statement(ast : &StatementAst, code : &mut Vec<vm::Operator>) {
    match ast {
        StatementAst::Exp(exp_ast) => compile(exp_ast, code),
        StatementAst::Assign(name, exp_ast) => (),
    }
}

pub fn compile_block(ast : &BlockAst, code : &mut Vec<vm::Operator>) {

}