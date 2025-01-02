use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::process::exit;

mod Lexer;

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

#[derive(std::fmt::Debug, Clone)]
enum Literal {
    Int(i32),
    Txt(String),
    Real(f64),
    Bool(bool),
    Unit,
}

#[derive(std::fmt::Debug, Clone)]
struct ADT {
    name: String,
    adt_type: Type,
    value: Box<Expr>,
}

#[derive(std::fmt::Debug, Clone)]
struct Closure {
    param: String,
    def: Box<Expr>,
    cl_type: Type,
}

#[derive(std::fmt::Debug)]
struct App {
    argument: Box<Expr>,
    action: Box<Expr>,
}

#[derive(std::fmt::Debug, Clone)]
enum Expr {
    ADT(ADT),
    Literal(Literal),
    ClosureExpr(Closure),
    App(Box<Expr>, Box<Expr>),
    Magic(MagicExpr),
}

#[derive(std::fmt::Debug, Clone)]
enum MagicExpr {
    AddX,
}

#[derive(std::fmt::Debug)]
struct Def {
    name: String,
    def_type: Type,
    body: Box<Expr>,
}

fn parse_type(tokens: &Vec<Token>, idx: &mut usize) -> Type {
    let token: &Token = &tokens[*idx];
    let new_type;

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
    match &tokens[*idx + 1] {
        Token::FnArrow => {
            *idx += 2;
            println!("INFO: Handle fn type, with id: {}", idx);
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

fn parse_closure(
    tokens: &Vec<Token>,
    idx: &mut usize,
    expr_type: &Type,
    defs: &mut Vec<Def>,
) -> Closure {
    if Token::FnSlash != tokens[*idx] {
        eprintln!(
            "SYNTAX ERROR: Expected a closure expression which must begin with \"\\\", but found: {:?}",
            tokens[*idx]
        );
        exit(1);
    } else {
        *idx += 1;
    }

    let mut args: Vec<String> = Vec::with_capacity(5);

    // Parsing args
    while let Token::Name(a) = &tokens[*idx] {
        args.push(a.clone());
        *idx += 1;
    }

    if args.is_empty() {
        eprintln!("SYNTAX ERROR: Expected closure arguments but found none");
        exit(1);
    }

    if Token::Equal != tokens[*idx] {
        eprintln!("SYNTAX ERROR: Expected closure definition which must begin with \"=\", but found: {:?}", tokens[*idx]);
        exit(1);
    } else {
        *idx += 1;
    }

    let expr = parse_expr(tokens, idx, &defs, None);

    // TODO: We assume the closure has one argument for now
    Closure {
        param: args[0].clone(),
        def: expr,
        cl_type: expr_type.clone(),
    }
}

fn parse_expr(
    tokens: &Vec<Token>,
    idx: &mut usize,
    defs: &Vec<Def>,
    expected_type: Option<Box<Type>>,
) -> Box<Expr> {
    // NOTE: We assume that we can come here only after some definition like a closure definition
    // or some other definition that is not at global level
    // NOTE: Because of this, we will not check types. We assume they are resolved at global scope
    // or at a let scope or the type is inferrable.
    // NOTE We assume every name is either a new name from a let expression or lambda expression or
    // is in the global scope. If not, we panic
    println!("Current token is: {:?}", tokens[*idx]);
    match &tokens[*idx] {
        Token::Int(a) => return Box::new(Expr::Literal(Literal::Int(*a))),
        Token::Real(a) => return Box::new(Expr::Literal(Literal::Real(*a))),
        Token::Name(var) => {
            for def in defs {
                if &def.name == var {
                    *idx += 1;
                    match &def.def_type {
                        Type::Int => {
                            if let Expr::Literal(Literal::Int(num)) = *def.body {
                                return Box::new(Expr::Literal(Literal::Int(num)));
                            } else {
                                eprintln!("PARSE-ERROR: Expected an integer due to type Int");
                                exit(1);
                            }
                        }
                        Type::Real => {
                            if let Expr::Literal(Literal::Real(num)) = *def.body {
                                return Box::new(Expr::Literal(Literal::Real(num)));
                            } else {
                                eprintln!("PARSE-ERROR: Expected a real number due to type Real");
                                exit(1);
                            }
                        }
                        Type::Bool => {
                            if let Expr::Literal(Literal::Bool(t)) = *def.body {
                                return Box::new(Expr::Literal(Literal::Bool(t)));
                            } else {
                                eprintln!("PARSE-ERROR: Expected a boolean due to type Bool");
                                exit(1);
                            }
                        }
                        Type::Unit => {
                            if let Expr::Literal(Literal::Unit) = *def.body {
                                return Box::new(Expr::Literal(Literal::Unit));
                            } else {
                                eprintln!("PARSE-ERROR: Expected unit due to type Unit");
                                exit(1);
                            }
                        }
                        // Type::Txt => {
                        //     if let Expr::Literal(Literal::Txt(text)) = *def.body {
                        //         return Box::new(Expr::Literal(Literal::Txt(text)));
                        //     } else {
                        //         eprintln!("PARSE-ERROR: Expected a real number due to type Real");
                        //         exit(1);
                        //     }
                        // }
                        Type::Any => {
                            eprintln!(
                                "TODO: The expression has type Any, which cannot be resolved."
                            );
                            exit(1);
                        }
                        Type::IO(_) => {
                            eprintln!(
                                "TODO: The expression has type IO, which is not handled yet."
                            );
                            exit(1);
                        }
                        Type::Fn(domain, _codomain) => {
                            parse_expr(tokens, idx, defs, Some(domain.clone()));
                        }
                        _ => {
                            todo!();
                        }
                    }
                    return def.body.clone();
                }
            }
        }
        _ => {
            eprintln!(
                "PARSE-ERROR: Could not handle the token of type: {:?}. Expected an expression here.",
                tokens[*idx]
            );
            exit(1);
        }
    }
    eprintln!(
        "PARSE-ERROR: Could not handle the token of type: {:?}. Likely because it was a name not found in global scope.",
        tokens[*idx]
    );
    exit(1);
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

fn parse(tokens: Vec<Token>, defs: &mut Vec<Def>, mut view: usize) {
    let mut name = String::new();
    let mut expr_type: Type = Type::Any;

    while view < tokens.len() {
        println!("CURRENT TOKEN IS: {:?} WITH ID: {}", tokens[view], view);
        match &tokens[view] {
            Token::Name(expr_name) => {
                name = expr_name.clone();
            }
            Token::Colon => {
                view += 1;
                expr_type = parse_type(&tokens, &mut view);
                println!("CURRENT TYPE IS: {:?}", expr_type);
            }
            Token::Equal => {
                view += 1;
                match &expr_type {
                    Type::Int | Type::Txt | Type::Real | Type::Bool | Type::Unit => {
                        let literal: Literal = parse_literal(&tokens, &mut view);
                        let expr = Expr::ADT(ADT {
                            name: name.clone(),
                            adt_type: expr_type.clone(),
                            value: Box::new(Expr::Literal(literal)),
                        });

                        defs.push(Def {
                            name: name.clone(),
                            def_type: expr_type.clone(),
                            body: Box::new(expr),
                        });
                    }
                    Type::Any => eprintln!("ERROR: expression {name} has type any!"),
                    Type::Fn(_, _) => {
                        let closure: Closure = parse_closure(&tokens, &mut view, &expr_type, defs);
                        defs.push(Def {
                            name: name.clone(),
                            def_type: expr_type.clone(),
                            body: Box::new(Expr::ClosureExpr(closure)),
                        });
                    }
                    Type::IO(_) => todo!(),
                }
            }
            Token::FnArrow => {
                eprintln!("ERROR: Function arrows are unreachable here.");
                exit(1);
            }
            token => eprintln!("ERROR: Found unexpected token {:?}!", token),
        }
        view += 1;
    }
}

fn main() -> std::io::Result<()> {
    Lexer::hello();
    let file = {
        let file = File::open("./examples/main.hm")?;
        BufReader::new(file)
            .bytes()
            .filter_map(|b| b.ok())
            .map(|b| b as char)
    };

    let tokens: Vec<Token> = tokenize(file);

    for t in &tokens {
        println!("{:?}", t);
    }

    let add = Def {
        name: "add".to_string(),
        def_type: Type::Fn(
            Box::new(Type::Int),
            Box::new(Type::Fn(Box::new(Type::Int), Box::new(Type::Int))),
        ),
        body: Box::new(Expr::ClosureExpr(Closure {
            param: "x".to_string(),
            cl_type: Type::Fn(
                Box::new(Type::Int),
                Box::new(Type::Fn(Box::new(Type::Int), Box::new(Type::Int))),
            ),
            def: Box::new(Expr::ClosureExpr(Closure {
                param: "x".to_string(), // First parameter
                cl_type: Type::Fn(
                    Box::new(Type::Int),
                    Box::new(Type::Fn(Box::new(Type::Int), Box::new(Type::Int))),
                ),
                def: Box::new(Expr::ClosureExpr(Closure {
                    param: "y".to_string(), // Second param
                    def: Box::new(Expr::Magic(MagicExpr::AddX)),
                    cl_type: Type::Fn(Box::new(Type::Int), Box::new(Type::Int)),
                })),
            })),
        })),
    };

    let defs = {
        let mut defs = Vec::<Def>::new();
        defs.push(add);
        parse(tokens, &mut defs, 0);
        defs
    };

    // for expr in exprs {
    //     println!("{:?}", expr);
    // }

    Ok(())
}
