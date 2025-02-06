use std::collections::HashMap;

#[derive(Debug)]
enum Command {
    Increment,
    Decrement,
    MoveRight,
    MoveLeft,
    ConditionalJumpRight,
    ConditionalJumpLeft,
    Print,
    Read,
}

pub struct Program {
    commands: Vec<Command>,
    jump_map: HashMap<usize, usize>,
}

impl Program {
    pub fn run(self) -> String {
        let mut cell_pointer: usize = 0;
        let mut command_pointer: usize = 0;

        let mut cells: Vec<u8> = vec![0; 30_000];
        let mut output = Vec::<u8>::new();

        loop {
            match self.commands.get(command_pointer) {
                None => break,
                Some(Command::Increment) => {
                    let cell = cells.get_mut(cell_pointer).expect("cell out of bounds");
                    *cell = cell.wrapping_add(1);
                    
                },
                Some(Command::Decrement) => {
                    let cell = cells.get_mut(cell_pointer).expect("cell out of bounds");
                    *cell = cell.wrapping_sub(1);
                },
                Some(Command::MoveRight) => {
                    cell_pointer += 1;
                },
                Some(Command::MoveLeft) => {
                    cell_pointer -= 1;
                },
                Some(Command::ConditionalJumpRight) => {
                    if *cells.get(cell_pointer).expect("cell out of bounds") == 0 {
                        command_pointer = *self.jump_map.get(&command_pointer).expect("jump failed")
                    }
                },
                Some(Command::ConditionalJumpLeft) => {
                    if *cells.get(cell_pointer).expect("cell out of bounds") != 0 {
                        command_pointer = *self.jump_map.get(&command_pointer).expect("jump failed")
                    }
                },
                Some(Command::Print) => {
                    output.push(*cells.get(cell_pointer).expect("cell out of bounds"));
                }
                _ => todo!()
            }
            command_pointer += 1;
        }

        String::from_utf8(output).expect("Invalid UTF-8")
    }
}

impl From<&str> for Program {
    fn from(source_code: &str) -> Self {
        let mut commands = Vec::<Command>::with_capacity(source_code.len());

        let mut jump_map = HashMap::new();
        let mut jump_stack = Vec::<usize>::new();

        for char in source_code.chars() {
            match char {
                '+' => commands.push(Command::Increment),
                '-' => commands.push(Command::Decrement),
                '>' => commands.push(Command::MoveRight),
                '<' => commands.push(Command::MoveLeft),
                '[' => {
                    jump_stack.push(commands.len());
                    commands.push(Command::ConditionalJumpRight)
                }
                ']' => {
                    if let Some(jump_origin) = jump_stack.pop() {
                        let jump_destination = commands.len();
                        _ = jump_map.try_reserve(2);
                        jump_map.insert(jump_origin, jump_destination);
                        jump_map.insert(jump_destination, jump_origin);
                        commands.push(Command::ConditionalJumpLeft)
                    } else {
                        panic!("mismatched ]")
                    }
                }
                '.' => commands.push(Command::Print),
                ',' => commands.push(Command::Read),
                unknown_char => panic!("unexpected char '{}' ({})", unknown_char, unknown_char as u8),
            }
        }
        Program { commands, jump_map }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_down() {
        let program = Program::from("+[--------->+<]>.-.-.-.-.-.-.-.-.");
        assert_eq!(program.run(), "987654321");
    }

    #[test]
    fn hello_world() {
        let program = Program::from("++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.");
        assert_eq!(program.run(), "Hello World!\n");
    }

    #[test]
    fn quine() {
        let quine = "-->+++>+>+>+>+++++>++>++>->+++>++>+>>>>>>>>>>>>>>>>->++++>>>>->+++>+++>+++>+++>+++>+++>+>+>>>->->>++++>+>>>>->>++++>+>+>>->->++>++>++>++++>+>++>->++>++++>+>+>++>++>->->++>++>++++>+>+>>>>>->>->>++++>++>++>++++>>>>>->>>>>+++>->++++>->->->+++>>>+>+>+++>+>++++>>+++>->>>>>->>>++++>++>++>+>+++>->++++>>->->+++>+>+++>+>++++>>>+++>->++++>>->->++>++++>++>++++>>++[-[->>+[>]++[<]<]>>+[>]<--[++>++++>]+[<]<<++]>>>[>]++++>++++[--[+>+>++++<<[-->>--<<[->-<[--->>+<<[+>+++<[+>>++<<]]]]]]>+++[>+++++++++++++++<-]>--.<<<]";
        let program = Program::from(quine);
        assert_eq!(program.run(), quine);
    }
}
