use crate::types::*;

pub fn interpret<'a>(program: Program) -> OperandType {
    let mut stack = Stack(Vec::new());

    for instruction in program {
        match instruction {
            Opcode::OpLoadVal(v) => {
                stack.push(v);
            },
            Opcode::OpReadVar(vn) => {
            },
            Opcode::OpWriteVar(vn) => {
            },
            Opcode::OpAdd => {
                let b = stack.pop();
                let a = stack.pop();
                stack.push(a + b);
            },
            Opcode::OpSubtract => {
                let b = stack.pop();
                let a = stack.pop();
                stack.push(a - b);
            },
            Opcode::OpMultiply => {
                let b = stack.pop();
                let a = stack.pop();
                stack.push(a * b);
            },
            Opcode::OpDivide => {
                let b = stack.pop();
                let a = stack.pop();
                stack.push(a / b);
            },
            Opcode::OpReturnValue => {
                break
            },
        }
    }

    stack.pop()
}