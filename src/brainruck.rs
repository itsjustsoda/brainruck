use std::path::Path;
use std::io::fs::File;
use std::os;

fn main() {
    let args = os::args();

    if args.len() == 1 {
        fail!("no arguments");
    }
    
    tokenize(parse_args(args));
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

#[deriving(Clone, Eq, Show, TotalEq)]
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
