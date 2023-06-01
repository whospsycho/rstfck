# 🧠 rstfck 🦀

This is a simple Brainfuck interpreter implemented in Rust. The interpreter provides functionality to execute Brainfuck programs and supports the basic Brainfuck commands.

## Features

- Increment and decrement the data pointer
- Increment and decrement the byte at the data pointer
- Output the byte at the data pointer
- Accept one byte of input, storing its value in the byte at the data pointer
- Looping support using `[` and `]` commands

## Usage

To use the Brainfuck interpreter, follow these steps:

1. Clone the repository or download the source code.
2. Build the interpreter using `cargo build`.
3. Run the interpreter with the Brainfuck program file as an argument: `cargo run <brainfuck_file>`.
   - The interpreter will execute the Brainfuck program and display the output.

## Example

Here's an example of running a Brainfuck program using the interpreter:

```bash
$ cargo run examples/hello_world.bf
```

The above command will execute the Brainfuck program hello_world.bf and display the output, which is typically "Hello World!".
