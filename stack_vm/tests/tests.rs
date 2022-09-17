use stack_vm::types::*;
use stack_vm::interpreter::*;

#[cfg(test)]
mod interpret_tests {
    use super::*;
    use Opcode::*;

    #[test]
    fn test_add() {
        let program = vec![
            OpLoadVal(1),
            OpLoadVal(2),
            OpAdd,
            OpReturnValue,
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
            OpLoadVal(10),
            OpLoadVal(2),
            OpAdd,
            OpLoadVal(7),
            OpSubtract,
            OpLoadVal(20),
            OpMultiply,
            OpLoadVal(4),
            OpDivide,
            OpReturnValue,
        ];

        assert_eq!(
            interpret(program),
            25,
            "result of add operation is incorrect"
        );
    }

    #[test]
    fn test_load_read_write() {
        let program = vec![
            OpLoadVal(1),
            OpWriteVar('x'),
            OpLoadVal(2),
            OpWriteVar('y'),
            OpReadVar('x'),
            OpLoadVal(1),
            OpAdd,
            OpReadVar('y'),
            OpMultiply,
            OpReturnValue,
        ];

        assert_eq!(
            interpret(program),
            4,
            "result of add operation is incorrect"
        );
    }
}