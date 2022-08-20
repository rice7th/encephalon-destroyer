use std::io::Read;
use std::env;

#[macro_use]
mod lib;
use lib::*;

#[allow(dead_code)]
#[inline]
fn lex(src_code: String) -> Vec<Token> {
    let mut token_stream: Vec<Token> = Vec::new();
    for token in src_code.chars() {
        match token {
            '+' => token_stream.push(Token::Plus),
            '-' => token_stream.push(Token::Minus),
            '.' => token_stream.push(Token::Dot),
            ',' => token_stream.push(Token::Comma),
            '[' => token_stream.push(Token::LeftBracket),
            ']' => token_stream.push(Token::RightBracket),
            '<' => token_stream.push(Token::LessThan),
            '>' => token_stream.push(Token::GreaterThan),
            _   => () // Comments
        }
    }
    return token_stream;
}

#[allow(dead_code)]
#[inline]
fn parse(token_stream: Vec<Token>) -> Result<Vec<Instruction>, String> {
    let mut program: Vec<Instruction> = Vec::new();
    let mut loop_stack: u16 = 0;
    let mut loop_position: usize = 0;
    for (index, instruction) in token_stream.iter().enumerate() {
        if loop_stack == 0 {
            match instruction {
                Token::Plus => program.push(Instruction::Increment),
                Token::Minus => program.push(Instruction::Decrement),
                Token::Dot => program.push(Instruction::Print),
                Token::Comma => program.push(Instruction::Read),
                Token::LessThan => program.push(Instruction::DecrementPtr),
                Token::GreaterThan => program.push(Instruction::IncrementPtr),
                Token::LeftBracket => {
                    loop_stack += 1;
                    loop_position = index;
                },
                Token::RightBracket => return Err(format!("unmatched ']' at {}", index)),
            }
        } else {
            match instruction {
                Token::LeftBracket => {
                    loop_stack += 1;
                },
                Token::RightBracket => {
                    loop_stack -= 1;
                    if loop_stack == 0 {
                        program.push(Instruction::Loop(match parse(token_stream[loop_position+1..index].to_vec()) {
                            Ok(prog) => prog,
                            Err(error) => return Err(error)
                        }));
                    }
                },
                _ => ()
            }
        }
    }
    if loop_stack != 0 {
        return Err(format!("unmatched '[' at {}", loop_position));
    }
    return Ok(program);
}

#[allow(dead_code)]
#[inline]
fn run(instructions: &Vec<Instruction>, array: &mut Vec<u8>, pointer: &mut usize) -> Result<(), String> {
    for instruction in instructions {
        // Why i need to dereference pointer?
        match instruction {
            Instruction::Increment => array[*pointer] = array[*pointer].wrapping_add(1), // wrapping
            Instruction::Decrement => array[*pointer] = array[*pointer].wrapping_sub(1),
            Instruction::IncrementPtr => *pointer += 1,
            Instruction::DecrementPtr => *pointer -= 1,
            Instruction::Print => print!("{}", array[*pointer] as char),
            Instruction::Read => {
                let mut input: [u8; 1] = [0];
                match std::io::stdin().read_exact(&mut input) {
                    Ok(_) => (),
                    Err(_) => return Err("Failed to read STDIN".to_string()),
                }
                array[*pointer] = input[0];
            },
            Instruction::Loop(thinking_about_a_name_for_this_variable) => while array[*pointer] != 0 {
                match run(&thinking_about_a_name_for_this_variable, array, pointer) {
                    Ok(_) => (),
                    Err(error) => return Err(error)
                } // This will return eventual Errors? Maybe??
            }
        }
    }
    Ok(())
}

#[allow(dead_code)]
#[inline]
fn main() {
    let args: Vec<String> = env::args().collect();

    #[allow(unused_assignments)]
    let mut file_path = String::new();

    #[allow(unused_assignments)]
    let mut array_size: usize = 2048;

    match args.get(1) {
        Some(arg) => match arg.as_str() {
            "-h" | "--help"    => show_help_and_crash!(),
            "-u" | "--usage"   => show_usage!(),
            "-v" | "--version" => show_version!(),
            "-i" | "--info"    => show_info!(),
            "-A" | "--array" => match args.get(2) {
                Some(array_size_arg) => {
                    array_size = match array_size_arg.trim().parse() {
                        Ok(num) => num,
                        Err(error) => raise_error!(error)
                    };
                    match args.get(3) {
                        Some(file) =>  file_path = file.to_string(),
                        None => raise_error!("File expected!")
                    }
                },
                None => raise_error!("Number expected!")
            }
            _ => { 
                file_path = arg.to_string();
                match args.get(2) {
                    Some(arg) => match arg.as_str() {
                        "-h" | "--help"    => show_help!(),
                        "-u" | "--usage"   => show_usage!(),
                        "-v" | "--version" => show_version!(),
                        "-i" | "--info"    => show_info!(),
                        "-A" | "--array" => match args.get(3) {
                            Some(array_size_arg) => array_size = match array_size_arg.trim().parse() {
                                Ok(num) => num,
                                Err(error) => raise_error!(error)
                            },
                            None => raise_error!("Number expected!")
                        }
                        _ => raise_error!(format!("Unknown flag {}", arg))

                    }
                    None => { }
                }
            }
        }
        None => {
            show_help!();
            raise_error!("Must provide a file");
        }
    }

    let file = match std::fs::read_to_string(file_path) {
        Ok(src) => src,
        Err(error) => match error.to_string().as_str() { // what even is this
            "stream did not contain valid UTF-8" => raise_error!("Please specify a file to run"),
            _ => raise_error!(error)
        },
    };

    let program = match parse(lex(file)) {
        Ok(prog) => prog,
        Err(error) => raise_error!(error)
    };

    println!("\t\t\t{array_size}");
    match run(&program, &mut vec![0u8; array_size], &mut 0usize) {
        Ok(_) => std::process::exit(0), // exit safely
        Err(error) => raise_error!(error),
    }
}