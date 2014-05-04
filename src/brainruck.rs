use std::path::Path;
use std::io::fs::File;
use std::os;

fn main() {
    let args = os::args();

    if args.len() != 2 {
        fail!("invalid argument");
    }

    let path: Path = Path::new(args[1]);
    let file: File = File::open(&path).unwrap();
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
    assert_eq!(tokenize("><+-.,[]".to_owned()).as_slice(),
               [Right, Left, Plus, Minus, Out, In, Jump, Loop].as_slice());

    assert_eq!(tokenize(">a<b+c-d. ,1[2]3 ".to_owned()).as_slice(),
               [Right, Left, Plus, Minus, Out, In, Jump, Loop].as_slice());
}
