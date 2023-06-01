// > 	Increment the data pointer by one (to point to the next cell to the right).
// < 	Decrement the data pointer by one (to point to the next cell to the left).
// + 	Increment the byte at the data pointer by one.
// - 	Decrement the byte at the data pointer by one.
// . 	Output the byte at the data pointer.
// , 	Accept one byte of input, storing its value in the byte at the data pointer.
// [ 	If the byte at the data pointer is zero, then instead of moving the instruction pointer forward to the next command, jump it forward to the command after the matching ] command.
// ] 	If the byte at the data pointer is nonzero, then instead of moving the instruction pointer forward to the next command, jump it back to the command after the matching [ command.[a]

use std::io;

struct Interpreter {
    cells: [u8; 30000],
    pointer: usize,
    brackets: Vec<usize>,
}

impl Interpreter {
    fn new() -> Interpreter {
        Interpreter {
            cells: [0; 30000],
            pointer: 0,
            brackets: Vec::new(),
        }
    }

    fn run(&mut self, code: &str) -> io::Result<()> {
        let mut i = 0;

        while i < code.len() {
            match code.chars().nth(i).unwrap() {
                '>' => self.pointer += 1,
                '<' => self.pointer -= 1,
                '+' => self.cells[self.pointer] = self.cells[self.pointer].wrapping_add(1),
                '-' => self.cells[self.pointer] = self.cells[self.pointer].wrapping_sub(1),
                '.' => print!("{}", self.cells[self.pointer] as char),
                ',' => {
                    let mut input = String::new();
                    io::stdin().read_line(&mut input)?;
                    self.cells[self.pointer] = input.chars().next().unwrap() as u8;
                },
                '[' => {
                    if self.cells[self.pointer] == 0 {
                        self.brackets.push(i);
                        while !self.brackets.is_empty() {
                            i += 1;
                            if code.chars().nth(i).unwrap() == '[' {
                                self.brackets.push(i);
                            } else if code.chars().nth(i).unwrap() == ']' {
                                self.brackets.pop();
                            }
                        }
                    } else {
                        self.brackets.push(i);
                    }
                },
                ']' => {
                    if self.cells[self.pointer] != 0 {
                        i = *self.brackets.last().unwrap();
                    } else {
                        self.brackets.pop();
                    }
                },
                _ => (),
            }
            i += 1;
        }

        Ok(())
    }
}

fn main() -> io::Result<()> {
    let args = std::env::args().collect::<Vec<String>>();
    let mut interpreter = Interpreter::new();
    let code = std::fs::read_to_string(&args[1])?;
    interpreter.run(&code)?;
    Ok(())
}