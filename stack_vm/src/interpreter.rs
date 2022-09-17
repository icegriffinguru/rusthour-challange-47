use crate::types::*;

pub fn interpret(program: Program) -> OperandType {
    let mut stack = Stack(Vec::new());
    let mut variable_registry = VariableRegistry::new();

    for instruction in program {
        match instruction {
            Opcode::OpLoadVal(v) => {
                stack.push(v);
            },
            Opcode::OpReadVar(vn) => {
                match variable_registry.get(&vn) {
                    Option::Some(v) => stack.push(*v),
                    Option::None => panic!("a given variable name does not exist in VariableRegistry"),
                }
            },
            Opcode::OpWriteVar(vn) => {
                variable_registry.insert(vn, stack.pop());
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