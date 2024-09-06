use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

enum Expression {
    Variable(HasmaTypes),                          // variable
    Abstraction(HasmaTypes, Box<Expression>),      // parameter + body
    Application(Box<Expression>, Box<Expression>), // function + argument
    DoBlock(Vec<Expression>),                      // do block
    LetBlock(Vec<Expression>),                     // let block
                                                   // if block
}

enum HasmaTypes {
    Txt(String),                                // Utf8 string
    Function(Box<HasmaTypes>, Box<Expression>), // function variable + function body
    Int(isize),                                 // signed integer
    Nat(usize),                                 // unsigned integer
    Unit,                                       // trivial type
    Tuple(Vec<HasmaTypes>),                     // tuple type
    Record(Vec<HasmaTypes>),                    // record type
    Enumeration(Vec<(String, HasmaTypes)>),     // enum tag + type
}

fn parse_expr<T>(file: T) -> Expression
where
    T: Iterator<Item = char>,
{
    let mut definitions: Vec<Expression> = Vec::new();
    let mut expr: Expression;
    let mut expr_string: String = String::new();

    for c in file {
        match c {
            ' ' | '\n' => {
                println!("{}", expr_string);
                expr_string = "".to_string();
            }
            _ => {
                expr_string += &c.to_string();
            }
        }
    }
    todo!()
}

fn parser<T>(file: T) -> Vec<Expression>
where
    T: Iterator<Item = char>,
{
    let mut definitions: Vec<Expression> = Vec::new();
    let expr: Expression = parse_expr(file);

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

    parser(file);

    Ok(())
}
