use std::env;

const ALPHABET: &str = "abcdefghijklmnopqrstuvwxyz";

macro_rules! propagate_none {
    ($expression: expr) => {
         match $expression { Some(result) => result, None => return None }
    }
}

#[derive(Debug,PartialEq,Clone)]
enum Symbol { E, LP, RP, LT, RT, P, M, NULL }

struct Token {
    value: String,
    symbol: Symbol,
}

impl Token {
    fn expression(value: String) -> Token { Token { value, symbol: Symbol::E } }
    fn symbol(symbol: Symbol) -> Token { Token { value: String::from(""), symbol } }
    fn is_trim(&self) -> bool { self.symbol == Symbol::RT || self.symbol == Symbol::LT }
}

fn tokenize(line: &str) -> Option<Vec<Token>> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut buffer: String = String::from("");

    for character in line.chars() {

        // Check if name
        if ALPHABET.contains(character) {
            buffer.push(character);
            continue;
        }

        // Push terminated name
        else if buffer.len() > 0 {
            let token: Token = Token::expression(buffer);
            tokens.push(token);
            buffer = String::from("");
        }

        // Check other characters
        match character {
            ' ' => {},
            '(' => tokens.push(Token::symbol(Symbol::LP)),
            ')' => tokens.push(Token::symbol(Symbol::RP)),
            '>' => tokens.push(Token::symbol(Symbol::LT)),
            '<' => tokens.push(Token::symbol(Symbol::RT)),
            '+' => tokens.push(Token::symbol(Symbol::P)),
            '-' => tokens.push(Token::symbol(Symbol::M)),
            _ => return None
        }
    }

    Some(tokens)
}

fn right_trim(string: &String) -> String {
    string.get(..string.len() - 1).unwrap().to_string()
}

fn left_trim(string: &String) -> String {
    string.get(1..).unwrap().to_string()
}

fn plus(left: String, right: String) -> String {
    left + &right
}

fn minus(left: String, right: String) -> String {
    match left.find(right.as_str()) {
        Some(index) => (
            left.get(0..index).unwrap().to_string() +
            left.get(index + right.len()..left.len()).unwrap()
        ),
        None => left.clone()
    }
}

fn parse_atomic(tokens: &mut Vec<Token>) -> Option<String> {
    let mut cursor: Token = propagate_none!(tokens.pop());
    let mut value: String = match cursor.symbol {
        Symbol::RP => propagate_none!(parse_molecular(tokens)),
        Symbol::E => cursor.value,
        _ => return None
    };
    while tokens.len() > 0 && tokens.last().unwrap().is_trim() {
        cursor = tokens.pop().unwrap();
        match cursor.symbol {
            Symbol::RT => value = right_trim(&value),
            Symbol::LT => value = left_trim(&value),
            _ => return None
        }
    }
    Some(value)
}

fn parse_molecular(tokens: &mut Vec<Token>) -> Option<String> {
    let mut right: String = propagate_none!(parse_atomic(tokens));
    let mut dominant_operator: Symbol = Symbol::NULL;
    loop {
        let operator: Symbol = match propagate_none!(tokens.pop()).symbol {
            Symbol::P => Symbol::P,
            Symbol::M => Symbol::M,
            Symbol::LP => return Some(right),
            _ => return None
        };

        // Check mixing operators
        if &dominant_operator != &Symbol::NULL {
            if &operator != &dominant_operator {
                return None
            }
        } else {
            dominant_operator = operator.clone();
        }

        let left: String = propagate_none!(parse_atomic(tokens));
        match operator {
            Symbol::P => right = plus(left, right),
            Symbol::M => right = minus(left, right),
            _ => return None
        }
    }
}

fn parse(mut tokens: Vec<Token>) -> Option<String> {
    match parse_atomic(&mut tokens) {
        Some(result) => {
            if tokens.len() > 0 { None }
            else { Some(result) }
        },
        None => None
    }
}

fn interpret(line: &str) {
    let tokens: Vec<Token> = match tokenize(line) {
        Some(tokens) => tokens,
        None => {
            println!("failed to tokenize!");
            return
        }
    };
    match parse(tokens) {
        Some(result) => println!("{}", result),
        None => println!("malformed!")
    };
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        println!("missing string to parse!");
        return;
    }
    interpret(&args[1]);
}
