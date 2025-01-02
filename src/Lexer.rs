#[derive(std::fmt::Debug, PartialEq)]
enum Lexeme {
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
    Comma,
}

pub struct Token {
    lex: Lexeme,
    line: usize,
    pos: usize,
}

static LINE: usize = 0;
static POS: usize = 0;

struct Tokens(Vec<Token>);

impl Tokens {
    fn new() -> Self {
        Tokens(Vec::new())
    }
    fn insert(&mut self, l: Lexeme) {
        let Tokens(items) = self;
        items.push(Token {
            lex: l,
            line: LINE,
            pos: POS,
        })
    }
}

pub fn tokenize<It>(input: It) -> Tokens
where
    It: Iterator<Item = char>,
{
    let mut tokens = Tokens::new();
    let mut s = String::new();

    let tokenize_string = |s: &mut String, tokens: &mut Tokens| {
        if let Ok(number) = s.parse::<i32>() {
            tokens.insert(Lexeme::Int(number));
        } else if let Ok(real) = s.parse::<f64>() {
            tokens.insert(Lexeme::Real(real));
        } else if !s.is_empty() {
            tokens.insert(Lexeme::Name(s.clone()));
        }

        *s = "".to_string();
    };

    for c in input {
        POS += 1;
        match c {
            '\n' => LINE += 1,
            t if t.is_whitespace() => tokenize_string(&mut s, &mut tokens),
            ';' => {
                tokenize_string(&mut s, &mut tokens);
                tokens.insert(Lexeme::SemiColon)
            }
            ':' => {
                tokenize_string(&mut s, &mut tokens);
                tokens.insert(Lexeme::Colon)
            }
            '(' => {
                tokenize_string(&mut s, &mut tokens);
                tokens.insert(Lexeme::ParenL)
            }
            ')' => {
                tokenize_string(&mut s, &mut tokens);
                tokens.insert(Lexeme::ParenR)
            }
            '"' => {
                tokenize_string(&mut s, &mut tokens);
                tokens.insert(Lexeme::Quote)
            }
            ',' => {
                tokenize_string(&mut s, &mut tokens);
                tokens.insert(Lexeme::Comma)
            }
            '=' => {
                tokenize_string(&mut s, &mut tokens);
                tokens.insert(Lexeme::Equal)
            }
            '>' => {
                if &s[0..] == "-" {
                    tokens.insert(Lexeme::FnArrow);
                    s = "".to_string();
                } else {
                    tokenize_string(&mut s, &mut tokens);
                    eprintln!("WARNING {LINE}: use of '-' and '>' as anything but the function constructor '->' is not supported yet");
                }
            }
            '\\' => tokens.insert(Lexeme::FnSlash),
            t if t.is_alphanumeric() || t == '_' || t == '-' || t == '-' => s.push(t),
            t => eprintln!("WARNING {LINE}: character: {t} not supported, ignoring it."),
        }
    }

    tokens
}
