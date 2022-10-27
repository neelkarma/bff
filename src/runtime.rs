use std::io::{stdin, Read};

use crate::parser::Token;

pub fn execute(tokens: Vec<Token>) {
    // Parse Vars
    let mut parse_ptr = 0;

    // Execution Vars
    let mut array = [0; 30000];
    let mut ptr = 0;

    while parse_ptr < tokens.len() {
        let token = &tokens[parse_ptr];
        match token {
            Token::Left => ptr -= 1,
            Token::Right => ptr += 1,
            Token::Increment => array[ptr] += 1,
            Token::Decrement => array[ptr] -= 1,
            Token::Output => print!("{}", array[ptr] as char),
            Token::Input => {
                let mut buf = [0];
                let _ = stdin().read(&mut buf).unwrap();
                array[ptr] = buf[0];
            }
            Token::LoopStart => {
                if array[ptr] == 0 {
                    // jump to after loop
                    let mut nesting = 0;
                    loop {
                        parse_ptr += 1;
                        match tokens[parse_ptr] {
                            Token::LoopStart => nesting += 1,
                            Token::LoopEnd => {
                                if nesting == 0 {
                                    break;
                                };
                                nesting -= 1;
                            }
                            _ => {}
                        };
                    }
                    parse_ptr += 1;
                    continue;
                }
            }
            Token::LoopEnd => {
                if array[ptr] != 0 {
                    // jump back to after beginning of loop
                    let mut nesting = 0;
                    loop {
                        parse_ptr -= 1;
                        match tokens[parse_ptr] {
                            Token::LoopEnd => nesting += 1,
                            Token::LoopStart => {
                                if nesting == 0 {
                                    break;
                                };
                                nesting -= 1;
                            }
                            _ => {}
                        }
                    }
                    parse_ptr += 1;
                    continue;
                }
            }
        }
        parse_ptr += 1;
    }
}
