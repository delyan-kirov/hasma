use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::process::exit;

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
    argument: Box<Expr>,
    action: Box<Expr>,
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
    Quote,
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
            '"' => {
                tokenize_string(&mut curr, &mut tokens);
                tokens.push(Token::Quote)
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
    let token: &Token = &tokens[*idx];
    let new_type;
    *idx += 1;
    new_type = match token {
        Token::Name(name) if &name[0..] == "Int" => Type::Int,
        Token::Name(name) if &name[0..] == "Txt" => Type::Txt,
        Token::Name(name) if &name[0..] == "Real" => Type::Real,
        Token::Name(name) if &name[0..] == "Bool" => Type::Bool,
        Token::Name(name) if &name[0..] == "IO" => {
            *idx += 1;
            let inner_type: Type = parse_type(tokens, idx);
            Type::IO(Box::new(inner_type))
        }
        Token::Name(name) if name.chars().next().is_none() => {
            eprintln!("ERROR: the type is somehow empty.");
            exit(1);
        }
        Token::Name(name) if name.chars().next().unwrap().is_lowercase() => {
            eprintln!(
                "SYNTAX ERROR: the type: {name} starts with a lowercase, which is not valid."
            );
            exit(1);
        }
        Token::Name(_adt) => {
            eprintln!("ERROR: Custom types are not supported yet!");
            exit(1);
        }
        Token::ParenL => {
            *idx += 1;
            match &tokens[*idx] {
                Token::ParenR => Type::Unit,
                _other_type => {
                    eprintln!("ERROR: nested types are not supported yet!");
                    exit(1);
                }
            }
        }
        token => {
            eprintln!("SYNTEX ERROR: Expected type, got {:?}", token);
            exit(1);
        }
    };
    match &tokens[*idx] {
        Token::FnArrow => {
            *idx += 1;
            let codomain: Type = parse_type(tokens, idx);
            Type::Fn(Box::new(new_type), Box::new(codomain))
        }
        Token::SemiColon | Token::Equal => new_type,
        token => {
            eprintln!(
                "SYNTEX ERROR: Expected a semi-colon or equals, found: {:?}",
                token
            );
            exit(1);
        }
    }
}

fn parse_closure(tokens: &Vec<Token>, idx: &mut usize) -> Closure {
    todo!()
}

fn parse_adt(tokens: &Vec<Token>, idx: &mut usize) -> ADT {
    todo!()
}

fn parse_app(tokens: &Vec<Token>, idx: &mut usize) -> App {
    todo!()
}

fn parse_literal(tokens: &Vec<Token>, idx: &mut usize) -> Literal {
    let token: &Token = &tokens[*idx];
    *idx += 1;
    match token {
        Token::Int(n) => Literal::Int(*n),
        Token::Real(r) => Literal::Real(*r),
        Token::Name(name) if &name[0..] == "True" || &name[0..] == "False" => {
            Literal::Bool(&name[0..] == "True")
        }
        Token::ParenL => {
            *idx += 1;
            match &tokens[*idx] {
                Token::ParenR => Literal::Unit,
                other_lit => {
                    eprintln!("ERROR: unsupported literal: {:?}", other_lit);
                    exit(1);
                }
            }
        }
        Token::Quote => {
            *idx += 1;
            match &tokens[*idx] {
                Token::Name(name) => {
                    *idx += 1;
                    match &tokens[*idx] {
                        Token::Quote => Literal::Txt(name.to_string()),
                        token => {
                            eprintln!("SYNTAX ERROR: expected a string literal ending with a \" found {:?}", token);
                            exit(1);
                        }
                    }
                }
                token => {
                    eprintln!(
                        "SYNTAX ERROR: expected a string literal ending with a \" found {:?}",
                        token
                    );
                    exit(1);
                }
            }
        }
        token => {
            // TODO It's better not to fail outright but rather return a custom error and then
            // print all parsing errors that were encountered.
            eprintln!("PARSE ERROR: The token {:?} is not a literal.", token);
            exit(1);
        }
    }
}

fn parse(tokens: Vec<Token>, exprs: &mut Vec<Expr>, mut view: usize) {
    let mut name = String::new();
    let mut expr_type: Type = Type::Any;

    match &tokens[view] {
        Token::Name(expr_name) => {
            view += 1;
            name = expr_name.clone();
        }
        Token::Colon => {
            view += 1;
            expr_type = parse_type(&tokens, &mut view);
        }
        Token::Equal => {
            view += 1;
            println!("HERE");
            match &expr_type {
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
            }
        }
        token => eprintln!("ERROR: Found unexpected token {:?}!", token),
    }
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

    for t in &tokens {
        println!("{:?}", t);
    }

    let exprs = {
        let mut exprs = Vec::<Expr>::new();
        parse(tokens, &mut exprs, 0);
        exprs
    };

    for expr in exprs {
        println!("{:?}", expr);
    }

    Ok(())
}
