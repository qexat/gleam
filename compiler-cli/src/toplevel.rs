use ecow::EcoString;
use gleam_core::{error::Error, parse, parse::lexer};
use std::io::{self, Write};
use termcolor::{Buffer, Color, ColorSpec, WriteColor};

use crate::cli::stdout_buffer_writer;

pub fn command() -> Result<(), Error> {
    let buffer_writer = stdout_buffer_writer();

    {
        let mut buffer = buffer_writer.buffer();

        buffer
            .set_color(
                ColorSpec::new()
                    .set_intense(true)
                    .set_fg(Some(Color::Magenta)),
            )
            .expect("could not set buffer's color");

        writeln!(buffer, "Gleam experimental toplevel").expect("failed to write toplevel premise");
        buffer_writer
            .print(&buffer)
            .expect("failed to print toplevel premise");
    }

    loop {
        let mut input_line = String::new();
        let mut buffer = buffer_writer.buffer();

        buffer
            .set_color(
                ColorSpec::new()
                    .set_intense(true)
                    .set_fg(Some(Color::Magenta)),
            )
            .expect("could not set buffer's color");

        write!(buffer, ">>> ").expect("could not write toplevel prompt");

        buffer
            .set_color(&ColorSpec::new())
            .expect("failed to reset color");

        buffer_writer
            .print(&buffer)
            .expect("could not print from buffer");
        io::stdout().flush().expect("failed to flush stdout");

        let _ = io::stdin()
            .read_line(&mut input_line)
            .expect("failed input");

        let processed_input_line = input_line.trim_end();
        let tokens = lexer::make_tokenizer(processed_input_line);
        let mut parser = parse::Parser::new(tokens);

        match parser.parse_statement() {
            Err(parse_error) => {
                let mut error_buffer = Buffer::ansi();
                let error = Error::Parse {
                    path: "<stdin>".into(),
                    src: EcoString::from(processed_input_line),
                    error: parse_error,
                };

                error.pretty(&mut error_buffer);

                io::stderr()
                    .write_all(error_buffer.as_slice())
                    .expect("failed to write to stderr");
            }
            Ok(value) => {
                println!("{:?}", value);
            }
        }
    }
}
