use stack_vm::types::*;
use stack_vm::interpreter::*;

#[cfg(test)]
mod interpret_tests {
    use super::*;

    #[test]
    fn test_add() {
        let program = vec![
            Opcode::OpLoadVal(1),
            Opcode::OpLoadVal(2),
            Opcode::OpAdd,
            Opcode::OpReturnValue,
        ];

        assert_eq!(
            interpret(program),
            3,
            "result of add operation is incorrect"
        );
    }

    #[test]
    fn test_add_subtract_multiply_divide() {
        let program = vec![
            Opcode::OpLoadVal(10),
            Opcode::OpLoadVal(2),
            Opcode::OpAdd,
            Opcode::OpLoadVal(7),
            Opcode::OpSubtract,
            Opcode::OpLoadVal(20),
            Opcode::OpMultiply,
            Opcode::OpLoadVal(4),
            Opcode::OpDivide,
            Opcode::OpReturnValue,
        ];

        assert_eq!(
            interpret(program),
            25,
            "result of add operation is incorrect"
        );
    }
}