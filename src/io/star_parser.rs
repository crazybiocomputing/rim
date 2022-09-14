use std::collections::HashMap;

// Constants for state
enum STAR {
    NONE,
    SEPARATOR,
    COMMENT,
    DATABLOCK,
    TOKEN,
    TABLE,
    HEADER,
    STRING,
    WORD,
    NUMBER,
}

impl ToString for STAR {
    fn to_string(&self) -> String {
        match self {
            STAR::NONE => format!("NONE"),
            STAR::SEPARATOR => format!("SEPARATOR"),
            STAR::COMMENT => format!("COMMENT"),
            STAR::DATABLOCK => format!("DATABLOCK"),
            _ => todo!(),
        }
    }
}

enum Value {
    Str(&'static str),
    Float(f64),
    Int(i32),
}

struct Token {
    t: STAR,
    v: String,
}

impl ToString for Token {
    fn to_string(&self) -> String {
        format!("type={}, value='{}'", self.t.to_string(), self.v)
    }
}

fn tokenize(txt: String) -> Vec<Token> {
    // Split into words. Separators one or more spaces, eols, tabs, etc.
    // TODO
    let v: Vec<&str> = txt
        .split(|c: char| !(c.is_alphanumeric()))
        .filter(|s| !s.is_empty())
        .collect();
    let mut toks = Vec::new();
    toks.push(Token {
        t: STAR::DATABLOCK,
        v: String::from("config"),
    });
    toks
}

fn parse_tokens(_toks: Vec<Token>) -> HashMap<String, Value> {
    let params = HashMap::from([
        ("Mercury".to_string(), Value::Str("yellow")),
        ("Venus".to_string(), Value::Float(0.7)),
        ("Earth".to_string(), Value::Str("blue")),
        ("Mars".to_string(), Value::Int(15)),
    ]);

    params
}

pub fn parse_star(txt: String) {
    println!("Parsing... (TODO)\n{}", txt);

    // First Pass
    let tokens: Vec<Token> = tokenize(txt);
    println!(
        "Tokenize {}",
        tokens
            .iter()
            .fold(String::new(), |acc, arg| acc + &arg.to_string())
    );

    // Second Pass -Parse
    let _structure: HashMap<String, Value> = parse_tokens(tokens); // parse(tokens);

    // println!("{}",structure);
    println!("End");
    // return structure;
}
