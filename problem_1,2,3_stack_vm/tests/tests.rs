use stack_vm::types::*;
use stack_vm::interpreter::*;

#[cfg(test)]
mod interpret_tests {
    use super::*;
    use Opcode::*;

    ////////////////////////////////////////////////////////////////////////////////////////////////////////////////
    // tests for Problem 1
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
            "result is incorrect"
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
            "result is incorrect"
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
            "result is incorrect"
        );
    }


    ////////////////////////////////////////////////////////////////////////////////////////////////////////////////
    // tests for Problem 2
    #[test]
    fn test_one_depth_loop() {
        // calculate sum of 1..100
        let program = vec![
            OpLoadVal(100),
            OpWriteVar('x'),
            OpLoadVal(0),
            OpWriteVar('s'),

            // start loop
            OpStartLoop,
            // s = x + s
            OpReadVar('x'),
            OpReadVar('s'),
            OpAdd,
            OpWriteVar('s'),

            // x = x - 1
            OpReadVar('x'),
            OpLoadVal(1),
            OpSubtract,
            OpWriteVar('x'),

            // check variable 'x'
            // if it equals to 0, the loop will end
            OpEndLoop('x'),

            OpReadVar('s'),
            OpReturnValue,
        ];

        assert_eq!(
            interpret(program),
            5050,
            "result is incorrect"
        );
    }

    #[test]
    fn test_two_depth_loop() {
        // double summation counting
        /*
            1
            1 + 2
            1 + 2 + 3
            ...
            1 + 2 + 3 + ... + 10
         */
        let program = vec![
            OpLoadVal(10),
            OpWriteVar('x'),
            OpLoadVal(0),
            OpWriteVar('s'),

            // start first loop
            OpStartLoop,
            // y = x
            OpReadVar('x'),
            OpWriteVar('y'),

            // start second loop
            OpStartLoop,
            // s = s + y
            OpReadVar('s'),
            OpReadVar('y'),
            OpAdd,
            OpWriteVar('s'),
            // y = y - 1
            OpReadVar('y'),
            OpLoadVal(1),
            OpSubtract,
            OpWriteVar('y'),
            OpEndLoop('y'),

            // x = x - 1
            OpReadVar('x'),
            OpLoadVal(1),
            OpSubtract,
            OpWriteVar('x'),

            // check variable 'x'
            // if it equals to 0, the loop will end
            OpEndLoop('x'),

            OpReadVar('s'),
            OpReturnValue,
        ];

        assert_eq!(
            interpret(program),
            220,
            "result is incorrect"
        );
    }
}