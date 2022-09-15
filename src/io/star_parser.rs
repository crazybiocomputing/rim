//
//  RIM - Rust Image
//  Copyright (C) 2022  Jean-Christophe Taveau.
//
//  This file is part of RIM
//
// This program is free software: you can redistribute it and/or modify it
// under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.
//
//  You should have received a copy of the GNU General Public License
//  along with RIM.  If not, see <http://www.gnu.org/licenses/>.
 
 
use std::collections::HashMap;

#[allow(dead_code)]

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

struct Table {
  header: Vec<String>,
  data: Vec<Value>
}

struct Attribute {
  key: String,
  value: Value
}

struct Category {
  name: String,
  attributes: Vec<Atribute>
}

struct Block {
  name: String,
  categories: Vec<Category>
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
    let _v: Vec<&str> = txt
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

/// Parse Star files
///
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
