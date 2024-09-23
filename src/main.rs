use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

#[derive(std::fmt::Debug, Clone)]
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
struct ADT {
    name: String,
    adt_type: Type,
    value: Box<Expr>,
}

#[derive(std::fmt::Debug)]
struct Closure {
    name: String,
    param: String,
    def: Box<Expr>,
    cl_type: Type,
}

#[derive(std::fmt::Debug)]
struct App {
    action: Box<Expr>,
    argument: Box<Expr>,
}

#[derive(std::fmt::Debug)]
enum Expr {
    ADT(ADT),
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

// fn parse_expr<It>(tokens: &mut It) -> Expr
// where
//     It: Iterator<Item = Token>,
// {
//     todo!()
// }

fn parse_type(tokens: &Vec<Token>, idx: &mut usize) -> Type {
    todo!()
}

fn parse_closure(tokens: &Vec<Token>, idx: &mut usize) -> Closure {
    todo!()
}

fn parse_adt<It>(tokens: &mut It) -> ADT
where
    It: Iterator<Item = Token>,
{
    todo!()
}

fn parse_app<It>(tokens: &mut It) -> App
where
    It: Iterator<Item = Token>,
{
    todo!()
}

fn parse_literal(tokens: &Vec<Token>, idx: &mut usize) -> Literal {
    todo!()
}

fn parse(tokens: Vec<Token>) -> Vec<Expr> {
    let mut exprs = Vec::<Expr>::new();
    let mut name = String::new();
    let mut expr_type: Type = Type::Any;
    let mut view: usize = 0;

    for (i, _) in tokens.iter().enumerate() {
        match &tokens[i] {
            Token::Name(expr_name) => name = expr_name.clone(),
            Token::Colon => expr_type = parse_type(&tokens, &mut view),
            Token::Equal => match &expr_type {
                Type::Int | Type::Txt | Type::Real | Type::Bool | Type::Unit | Type::IO(_) => {
                    let literal: Literal = parse_literal(&tokens, &mut view);
                    let expr = Expr::ADT(ADT {
                        name: name.clone(),
                        adt_type: expr_type.clone(),
                        value: Box::new(Expr::Literal(literal)),
                    });
                    exprs.push(expr);
                }
                Type::Any => eprintln!("ERROR: expression {name} has type any!"),
                Type::Fn(_, _) => {
                    let closure: Closure = parse_closure(&tokens, &mut view);
                    exprs.push(Expr::Closure(closure));
                }
            },
            token => eprintln!("ERROR: Found unexpected token {:?}!", token),
        }
        view += 1;
    }
    todo!()
}
// Name(String),
// Param(String),
// Def(Box<Expr>),
// Type(Type),

fn main() -> std::io::Result<()> {
    let file = {
        let file = File::open("./examples/main.hm")?;
        BufReader::new(file)
            .bytes()
            .filter_map(|b| b.ok())
            .map(|b| b as char)
    };

    let tokens = tokenize(file);

    for t in &tokens {
        println!("{:?}", t);
    }

    let exprs = parse(tokens);

    Ok(())
}
