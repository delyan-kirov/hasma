use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

#[derive(std::fmt::Debug)]
enum Type {
    Int,
    Txt,
    Real,
    Bool,
    Unit,
    IO(Box<Type>),
    Fn(Box<Type>, Box<Type>),
    Any,
}

#[derive(std::fmt::Debug)]
enum Literal {
    Int(i32),
    Txt(String),
    Real(f64),
    Bool(bool),
    Unit,
}

#[derive(std::fmt::Debug)]
enum Binding {
    Name(String),
    Type(Type),
}

#[derive(std::fmt::Debug)]
enum Closure {
    Param(Binding),
    Def(Box<Expr>),
    Type(Type),
}

#[derive(std::fmt::Debug)]
enum Expr {
    Var(Binding),
    Literal(Literal),
    Closure(Closure),
    App(Box<Expr>, Box<Expr>),
}

#[derive(std::fmt::Debug)]
enum Token {
    Int(i32),
    Name(String),
    Real(f64),
    Equal,
    FnArrow,
    ParenL,
    ParenR,
    Colon,
    SemiColon,
    FnSlash,
}

fn tokenize<It>(input: It) -> Vec<Token>
where
    It: Iterator<Item = char>,
{
    let mut tokens = Vec::<Token>::new();
    let mut curr = String::new();
    let mut line: usize = 0;

    let tokenize_string = |s: &mut String, tokens: &mut Vec<Token>| {
        if let Ok(number) = s.parse::<i32>() {
            tokens.push(Token::Int(number));
        } else if let Ok(real) = s.parse::<f64>() {
            tokens.push(Token::Real(real));
        } else if !s.is_empty() {
            tokens.push(Token::Name(s.clone()));
        }

        *s = "".to_string();
    };

    for c in input {
        match c {
            '\n' => line += 1,
            t if t.is_whitespace() => tokenize_string(&mut curr, &mut tokens),
            ';' => {
                tokenize_string(&mut curr, &mut tokens);
                tokens.push(Token::SemiColon)
            }
            ':' => {
                tokenize_string(&mut curr, &mut tokens);
                tokens.push(Token::Colon)
            }
            '(' => {
                tokenize_string(&mut curr, &mut tokens);
                tokens.push(Token::ParenL)
            }
            ')' => {
                tokenize_string(&mut curr, &mut tokens);
                tokens.push(Token::ParenR)
            }
            '=' => {
                tokenize_string(&mut curr, &mut tokens);
                tokens.push(Token::Equal)
            }
            '>' => {
                if &curr[0..] == "-" {
                    tokens.push(Token::FnArrow);
                    curr = "".to_string();
                } else {
                    tokenize_string(&mut curr, &mut tokens);
                    eprintln!("WARNING {line}: use of '-' and '>' as anything but the function constructor '->' is not supported yet");
                }
            }
            '\\' => tokens.push(Token::FnSlash),
            t if t.is_alphanumeric() || t == '_' || t == '-' || t == '-' => curr.push(t),
            t => eprintln!("WARNING {line}: character: {t} not supported, ignoring it."),
        }
    }

    tokens
}

fn parse(tokens: Vec<Token>) -> Vec<Expr> {
    todo!()
}

fn main() -> std::io::Result<()> {
    let file = {
        let file = File::open("./examples/main.hm")?;
        BufReader::new(file)
            .bytes()
            .filter_map(|b| b.ok())
            .map(|b| b as char)
    };

    let tokens = tokenize(file);

    for t in tokens {
        println!("{:?}", t);
    }

    Ok(())
}
