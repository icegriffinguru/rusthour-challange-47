use crate::types::*;

pub fn interpret(program: Program) -> OperandType {
    use Opcode::*;

    let mut stack = Stack(Vec::new());
    // temporarily store naem and value of variables in HashMap
    let mut variable_registry = VariableRegistry::new();

    let mut i = 0;
    while i < program.len() {
        let instruction = program[i];

        match instruction {
            // load value
            OpLoadVal(v) => {
                stack.push(v);
            },
            // read variable from VariableRegistry
            OpReadVar(vn) => {
                match variable_registry.get(&vn) {
                    Option::Some(v) => stack.push(*v),
                    Option::None => panic!("a given variable name does not exist in VariableRegistry"),
                }
            },
            // write variable to VariableRegistry
            OpWriteVar(vn) => {
                variable_registry.insert(vn, stack.pop());
            },
            OpAdd => {
                let b = stack.pop();
                let a = stack.pop();
                stack.push(a + b);
            },
            OpSubtract => {
                let b = stack.pop();
                let a = stack.pop();
                stack.push(a - b);
            },
            OpMultiply => {
                let b = stack.pop();
                let a = stack.pop();
                stack.push(a * b);
            },
            OpDivide => {
                let b = stack.pop();
                let a = stack.pop();
                stack.push(a / b);
            },
            OpStartLoop => {
            },
            // a loop will end if the given variable equals to 0
            OpEndLoop(vn) => {
                let mut j = i;
                // this variable will allow us to find start of the matching loop by skipping inner loops
                let mut loop_count = 1;

                // check if value of the loop variable is zero
                match variable_registry.get(&vn) {
                    Option::Some(v) => {
                        // if value of the loop variable is zero, end the loop
                        // if not, go to start of loop by changing i
                        if *v != 0 {
                            loop {
                                if j == 0 {
                                    panic!("cannot find matching OpStartLoop");
                                }
                                j -= 1;
            
                                let ins = program[j];
                                // if j finds OpEndLoop, it starts another inner loop
                                // if j finds OpStartLoop, it finishes another inner loop
                                match ins {
                                    OpStartLoop => loop_count -= 1,
                                    OpEndLoop(_) => loop_count += 1,
                                    _ => {},
                                }
            
                                // if loop_count reachs 0, it means j reachs a matching OpStartLoop
                                if loop_count == 0 {
                                    i = j;
                                    break;
                                }
                            }
                        }
                    },
                    Option::None => panic!("a given variable name does not exist in VariableRegistry"),
                }
            },
            OpReturnValue => {
                break
            },
        }

        i += 1;
    }

    stack.pop()
}