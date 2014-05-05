use std::os;
use std::str;
use std::fmt;

fn main() {
    let args = os::args();

    if args.len() == 1 {
        fail!("no arguments");
    }
    
    run(tokenize(parse_args(args)));
}

fn parse_args(args: ~[~str]) -> ~str {
    args[1]
}

fn tokenize(text: ~str) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    for c in text.chars() {
       match c {
           '>' => {tokens.push(Right)},
           '<' => {tokens.push(Left)},
           '+' => {tokens.push(Plus)},
           '-' => {tokens.push(Minus)},
           '.' => {tokens.push(Out)},
           ',' => {tokens.push(In)},
           '[' => {tokens.push(Jump)},
           ']' => {tokens.push(Loop)},
           _ => {}
       };
    }

    return tokens;
}

fn run(tokens: Vec<Token>) {
    let mut data: Vec<u8> = Vec::new();
    let mut jump_stack: Vec<uint> = Vec::new();

    let mut pointer: uint = 0;
    let mut token_index: uint = 0;

    let token_slice = tokens.as_slice();

    data.push(0);

    while token_index < tokens.len() {
        std::io::timer::sleep(10);
        for (i, t) in tokens.iter().enumerate() {
            if i == token_index {
                print!("\x1b[34m{:3}\x1b[0m", t);
            } else {
                print!("{:3}", t);
            }
        }
        print!("\n");

        let token = token_slice[token_index];
        match token {
            Right => {
                pointer += 1;
                if pointer == data.len() {
                    data.push(0);
                }

                token_index += 1;
            },

            Left => {
                if pointer == 0 {
                    fail!("index out of bounds");
                }

                pointer -= 1;
                token_index += 1;
            },

            Plus => {
                *data.get_mut(pointer) += 1;
                token_index += 1;
            },

            Minus => {
                *data.get_mut(pointer) -= 1;
                token_index += 1;
            },

            Out => {
                match  str::from_utf8(data.slice(pointer, pointer + 1)) {
                    Some(c) => {print!("{}", c)},
                    None => {}
                }

                token_index += 1;
            },

            Jump => {

                // Jump to end of the loop when the value at the pointer is zero.
                if *data.get_mut(pointer) == 0 {

                    let mut jump_count = 0;
                    loop {
                        token_index += 1;

                        if token_index > tokens.len() {
                            fail!("no matching ]");
                        }

                        match token_slice[token_index] {
                            Loop => {
                                if jump_count == 0 {
                                    token_index += 1;
                                    break;
                                } else {
                                    jump_count -= 1;
                                }
                            },

                            Jump => {
                                jump_count += 1;
                            },

                            _ => {}
                        }
                    }
                } else {
                    jump_stack.push(token_index);
                    token_index += 1;
                }
            },

            Loop => {
                match jump_stack.pop() {
                    Some(i) => {token_index = i},
                    None => {}
                }
            },
            
            _ => {}
        }

        print!("{} {} ", token, pointer);
        for (i, d) in data.iter().enumerate() {
            if i == pointer {
                print!("|\x1b[31m{:3}\x1b[0m", d);
            } else {
                print!("|{:3}", d);
            }
        }
        print!("|\n");
    }

    print!("\n");
}

#[deriving(Clone, Eq, TotalEq)]
enum Token {
    Right,
    Left,
    Plus,
    Minus,
    Out,
    In,
    Jump,
    Loop
}

impl fmt::Show for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f.buf, "{}", match *self {
            Right => '>',
            Left => '<',
            Plus => '+',
            Minus => '-',
            Out => '.',
            In => ',',
            Jump => '[',
            Loop => ']',
        })
    }
}

#[test]
fn testTokenizer() {
    // Check the tokenizer is getting the right outputs.
    assert_eq!(tokenize("><+-.,[]".to_owned()).as_slice(),
               [Right, Left, Plus, Minus, Out, In, Jump, Loop].as_slice());

    // Make sure junk is ignored.
    assert_eq!(tokenize(">a<b+c-d. ,1[2]3 ".to_owned()).as_slice(),
               [Right, Left, Plus, Minus, Out, In, Jump, Loop].as_slice());
}

#[test]
fn testArgParser() {
    assert_eq!(parse_args(["brainruck".to_owned(), "><+-.,[]".to_owned()].to_owned()),
               "><+-.,[]".to_owned());
}
