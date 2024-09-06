use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

enum Variable {
    Txt(String),
    Integer(i64),
    Natural(i64),
    Unit,
    Tuple(Box<Variable>, Box<Variable>),
    Record(Vec<Variable>, HasmaTypes),
    Enumeration(Vec<Expression>),
    Boolean(bool),
}

enum Expression {
    Variable(Variable, HasmaTypes),
    Abstraction {
        parameter: HasmaTypes,
        definition: Box<Expression>,
    },
    Application {
        hasma_type: HasmaTypes,
        parameter: Box<Expression>,
        definition: Box<Expression>,
    }, // function + argument
    DoBlock(Vec<Expression>),  // do block
    LetBlock(Vec<Expression>), // let block
    IfBlock(Vec<(Expression, Expression)>), // if block
                               // case block
}

enum HasmaTypes {
    Boolean,                                    // Boolean data type
    Txt,                                        // Utf8 string
    Function(Box<HasmaTypes>, Box<Expression>), // function variable + function body
    Int,                                        // signed integer
    Nat,                                        // unsigned integer
    Unit,                                       // trivial type
    Tuple(Box<HasmaTypes>, Box<HasmaTypes>),    // tuple type
    Record(Vec<HasmaTypes>),                    // record type
    Enumeration(Vec<(String, HasmaTypes)>),     // enum tag + type
    NoneDetermined,                             // type either undetemined or wrong
}

fn parser<T>(file: T, definitions: Vec<Expression>) -> Expression
where
    T: Iterator<Item = char>,
{
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

fn main() -> std::io::Result<()> {
    let file = {
        let file = File::open("./examples/main.hm")?;
        BufReader::new(file)
            .bytes()
            .filter_map(|b| b.ok())
            .map(|b| b as char)
    };

    let definitions: Vec<Expression> = Vec::new();
    let _ = parser(file, definitions);

    Ok(())
}
