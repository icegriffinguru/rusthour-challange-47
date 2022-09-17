use std::collections::HashMap;

pub type OperandType = i64;
pub type VariableNameType = char;

#[derive(Debug, Copy, Clone)]
pub enum Opcode {
    OpLoadVal(OperandType),
    OpReadVar(VariableNameType),
    OpWriteVar(VariableNameType),
    OpAdd,
    OpSubtract,
    OpMultiply,
    OpDivide,
    OpStartLoop,
    OpEndLoop(VariableNameType),
    OpReturnValue,
}

// stack implmentation
pub struct Stack(pub Vec<OperandType>);
impl Stack {
    pub fn push(&mut self, v: OperandType) {
        self.0.push(v);
    }

    pub fn pop(&mut self) -> OperandType{
        self.0.pop().expect("popped an empty stack")
    }
}

// register value of all variables into HashMap for easy retrieving
pub type VariableRegistry = HashMap<VariableNameType, OperandType>;

// set of instructions
pub type Program = Vec<Opcode>;