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
 
/*
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
*/
/*

JS code for parsing STAR files
----------------------------------
/*
 * STAR/STAR Parser
 * Jean-Christophe Taveau
 * 2021/11/15
 */

  // Constants for state
  const STAR = {
    NONE: 0,
    SEPARATOR: 1,
    COMMENT: 2,
    DATABLOCK: 3,
    TOKEN: 4,
    TABLE: 5,
    HEADER: 6,
    STRING: 7,
    WORD: 8,
    NUMBER: 9
  };
  Object.freeze(STAR);
  

/////////////////////::: T O K E N I Z E R ::://///////////////////

/**
 * STAR Tokenizer
 */
const tokenize = (txt) => {

  // Predicates
  const isDataBlock = (w) => (w.slice(0,5) === 'data_');
  const isTable = (w) => (w === 'loop_');
  const isFirst = (symbol) => w => (w[0] === symbol);
  const isComment = isFirst('#');
  const isToken = isFirst('_');
  const isMultiLine = isFirst(';');
  const isString = isFirst('\'');
  const isNumber = (w) => (!isNaN(Number(w)));
  const isSeparator = (w) => w.split('').every( ch => [' ','\t','\n'].includes(ch) );
  const isWord = (w) => true;
  
  // Create Basic Token
  const basicToken = (type) => (w,i,array) => [{type: type,v:w},i];
  
  // Create Numeric Token
  const numericToken = (type) => (w,i,array) => [{type: type,v:parseFloat(w)},i];
  
  // Create StringToken using Recursion
  const appendWord = (predicate,array,j,str='') => {
    let word = array[++j];
    str += word;
    return (predicate(word) === false) ? [j,str] : appendWord(predicate,array,j,str);
  }

  const stringToken = (type,predicate) => (w,i,array) => {
    let [j,str] = appendWord(predicate,array,i,w);
    // Remove leading delimiters
    const v = str.slice(1,str.length-1);
    return [{type,v},j];
  }

  const keywords = [
    {
      predicate: isDataBlock,
      newToken: (w,i,array) => [{type: STAR.DATABLOCK,v:w.slice(5)},i],
    },
    {
      predicate: isTable,
      newToken: basicToken(STAR.TABLE)
    },
    {
      predicate: isComment,
      newToken: stringToken(STAR.COMMENT,word => word[0] !== '\n')
    },
    {
      predicate: isSeparator,
      newToken: basicToken(STAR.SEPARATOR) 
    },
    {
      predicate: isToken,
      newToken: basicToken(STAR.TOKEN)
    },
    {
      predicate: isMultiLine,
      newToken: stringToken(STAR.STRING, word => (word[0] !== ';') )
    },
    {
      predicate: isNumber,
      newToken: numericToken(STAR.NUMBER)
    },
    {
      predicate: isString,
      newToken: stringToken(STAR.STRING, word => word[word.length-1] !== '\'')
    },
    {
      predicate: isWord,
      newToken: basicToken(STAR.WORD) 
    }
  ];

  const setToken = (words) => (index) => {
    let w = words[index];
    // Get Token corresponding to keyword
    const toks = keywords.reduce( (accu,kw) => {
      // const newTok = iif(kw,w,index,words).newToken();
      if (kw.predicate(w)) {
        accu.push(kw.newToken(w,index,words));
      }
      return accu;
    },[]);
    
    // Add new Token. Only the first one because the last one is always `STAR.WORD`
    return toks[0]; // keyword.newToken(w,index,words);
  };
  
  ///// M A I N /////
  const words = txt.split(/(\s+)/);
  let tokens = [];
  let index = 0;
  const setTokenAt = setToken(words);

  // TODO Use (tail) recursion
  while (index < words.length) {
    [tok,index] = setTokenAt(index);
    tokens.push(tok);
    index++;
  }
  console.info(tokens);
  return tokens;
}

/////////////////////::: P A R S E R ::://///////////////////

/**
 * Parsers
 */
const parseComment = (tok,obj) => {
  // Reset if STAR.TABLE
  if (obj._admin_.state === STAR.TABLE) {
    obj._admin_.next = [STAR.TOKEN,STAR.TABLE];
    obj._admin_.state = STAR.NONE;
    obj._admin_.current = null;
  }

  return obj;
}

const setHeader = (cat,attr,obj) => {
  if (cat in obj === false) {
    // Create Table
    obj[cat] = {
      header: [],
      rows: [[]]
    };
  }
  obj[cat].header.push(attr);
  obj._admin_.current = cat; // Current Category to fill in
  obj._admin_.next = [STAR.TOKEN,STAR.NUMBER,STAR.STRING,STAR.WORD];
  return obj;
}

/**
 * Parse nothing - Use for skipping token(s)
 *
 * @param {Token} - `tok` a token composed of 
 * @param {Object} - `obj` the Data Structure
 * @returns {object} - The UNmodified Data Structure
 *
 * @author Jean-Christophe Taveau
 */
 const parseNothing = (tok,obj) => obj;

/**
 * Set STAR Category
 *
 * @param {Token} - `tok` a token composed of 
 * @param {Object} - `obj` the Data Structure
 * @returns {object} - The modified Data Structure
 *
 * @author Jean-Christophe Taveau
 */
const setCategory = (cat,attr,obj) => {
  // Create object if needed
  if (cat in obj === false) {
    obj[cat] = {};
  }
  obj[cat][attr] = 0;
  obj._admin_.next = [STAR.NUMBER,STAR.STRING,STAR.WORD];
  obj._admin_.current = cat;
  obj._admin_.attr = attr;
  return obj;
}

/**
 * Parse STAR Token: parseHead or parseCat depending of context (STAR.TABLE or STAR.TOKEN)
 *
 * @param {Object} - `tok` a token composed of a type `type` and a value `v`.
 * @param {Object} - `obj` the Data Structure.
 * @returns {object} - The modified Data Structure.
 *
 * @author Jean-Christophe Taveau
 */
const parseToken = (tok,obj) => {
  // Remove the leading underscore `_` and Split `category` and `attribute` 
  let [cat,attr] = tok.v.slice(1).split('.'); 
  return (obj._admin_.state === STAR.TABLE) ? setHeader(cat,attr,obj) : setCategory(cat,attr,obj);
}

/**
 * Parse STAR Value in Token or in Table: setRowValue or parseCat depending of context (STAR.TABLE or STAR.TOKEN)
 *
 * @param {Object} - `tok` a token composed of a type `type` and a value `v`.
 * @param {Object} - `obj` the Data Structure.
 * @returns {object} - The modified Data Structure.
 *
 * @author Jean-Christophe Taveau
 */
const setRowValue = (tok,obj) => {
  const table = obj[obj._admin_.current];
  let last = table.rows.length-1;
  if (table.rows[last].length >= table.header.length) {
    table.rows.push([]); // Add a new row in the table
    last = table.rows.length-1; // Update
  }
  table.rows[last].push(tok.v);
  return obj;
}

const setTokenValue = (tok,obj) => {
  obj[obj._admin_.current][obj._admin_.attr] = tok.v;
  obj._admin_.current = null;
  obj._admin_.attr = null;
  return obj;
}

const parseValue = (tok,obj) => {
  return (obj._admin_.state === STAR.TABLE) ? setRowValue(tok,obj) : setTokenValue(tok,obj);
}

/**
 * Parse STAR Table
 *
 * @param {Object} - `tok` a token composed of a type `type` and a value `v`.
 * @param {Object} - `obj` the Data Structure.
 * @returns {object} - The modified Data Structure.
 *
 * @author Jean-Christophe Taveau
 */
const parseTable = (tok,obj) => {
  obj._admin_.state = STAR.TABLE;
  obj._admin_.next = [STAR.TOKEN];
  return obj;
}

/**
 * Parse STAR DataBlock
 *
 * @param {Object} - `tok` a token composed of a type `type` and a value `v`.
 * @param {Object} - `obj` the Data Structure.
 * @returns {object} - The modified Data Structure.
 *
 * @author Jean-Christophe Taveau
 */
const parseDataBlock = (tok,obj) => {
  obj._admin_.next = [STAR.TOKEN,STAR.TABLE]; // TOKEN, TABLE
  obj.datablock = tok.v;
  return obj;
}

/**
 * Parsers for the following tokens:
 *  - NONE: 0 ,
 *  - SEPARATOR: 1,
 *  - COMMENT: 2,
 *  - DATABLOCK: 3,
 *  - TOKEN: 4,
 *  - TABLE: 5,
 *  - HEADER: 6,
 *  - STRING: 7,
 *  - WORD: 8,
 *  - NUMBER: 9
 */
const parser = (toks) => {
  const  setters = [parseNothing,parseNothing,parseComment,parseDataBlock,parseToken,parseTable,parseNothing,parseValue,parseValue,parseValue];

  const model = toks.reduce( (accu,tok) => {
    if (accu._admin_.next.includes(tok.type)) {
      return setters[tok.type](tok,accu);
    }
    return accu;
  },{_admin_: {next: [STAR.DATABLOCK],state: STAR.NONE}});

  return model;
}

/// STAR Parser
 
const parseSTAR = (txt) => {

  // First Pass
  const tokens = tokenize(txt);

  // Second Pass -Parse
  let structure = parser(tokens); // parse(tokens);
  console.log(structure);
  console.info('End');
  return structure;
}



*/
