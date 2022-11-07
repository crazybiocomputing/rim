#[allow(non_snake_case)]
#[allow(unused_imports)]
#[allow(dead_code)]
mod io;
#[allow(unused_imports)]
use crate::io::star_reader;

// use std::env;
use std::fs::File;
use std::io::prelude::*;


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

// impl AddAssign for Value {
//     fn add_assign(&mut self, other: Self) {
//         *self = Self {
//             x: self.x + other.x,
//             y: self.y + other.y,
//         };
//     }
// }
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
  attributes: Vec<Attribute>
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


//functions

fn print_type_of<T>(_: &T) {
    std::any::type_name::<T>()
}


fn read_file(filepath:&str) -> String{
    let mut file = File::open(filepath).expect("file not found");
    let mut content = String::new();
    file.read_to_string(&mut content).expect("cannot read");
    // println!("{}",content);
    content
}

fn parse_star(txt: String) 
{
    
    let chariot :Vec<&str>= txt.split(['\n']).collect();
    let mut txt :Vec<String>= Vec::new();
    
    for i in chariot
    {
        let mut protoline=String::from("");
        for j in i.chars()
        {
            if j !='#' //skip comments
            {
                let s=j.to_string();
                protoline+= &s;
            }
            else
            {
                break;
            }
        }   
        let line=protoline.clone();
        // println!("{}",line);
        let length=&line.chars().count();
        if length > &0 //skip empty lines
        {
            txt.push(line);     
        }
    }
    // println!("reponse {:?}",txt);
    sort_file(txt);
}

fn sort_file(txt:Vec<String>)
{
    let mut categories:Vec<Category>= Vec::new();
    let mut first_word=String::from("");
    let mut first=true;
    let mut value=String::from("");
    let mut veclines:Vec<Vec<&str>>=Vec::new();
    let mut categorie=true;

    for i in txt
    {
        let mut line=i.clone();
        let length=&i.chars().count();
        if length>&0
        {
            let mut multiline=false;
            if i.chars().nth(0).unwrap().to_string()=="_"
            {

                let espace :Vec<&str>= line.split_whitespace().collect();
                let second_decoupe:Vec<&str>=espace[0].split(".").collect();
                let mut categorie_word=second_decoupe[0].to_string();
                let mut attribute_word=second_decoupe[1].to_string();
                if multiline==false
                {
                    if espace.len()>=2{
                        for i in espace{
                            let valeur= i.to_string();
                            value += &valeur;
                        for nom in categories
                            {
                            if categorie_word==nom.name
                                {
                                    let mut categorie=false;
                                    let mut the_value:Value=Value::value;
                                    let mut attribute_found=Attribute{ key:attribute_word, value:value};
                                    nom.attributes.push(attribute_found);
                                }
                            }
                            if categorie
                            {
                                let attribut_of_cat:Vec<Attribute>=Vec::new();
                                let mut attribute_found=Attribute{ key:attribute_word, value:value};
                                attribut_of_cat.push(attribute_found);
                                let new_cat= Category{ name: categorie_word , attributes: attribut_of_cat };
                                categories.push(new_cat);
                            }
                        }
                    }
                    else{
                        multiline=true;
                    }

                }
              
                if multiline==true{
                    if value.chars().last().unwrap().to_string()==";"
                    {
                        multiline=false;
                        value.remove(0);
                        value.pop();
                       
                    }
                }
            }
        
                // println!("{:?}  {}",veclines,value);
            }
        }
    }


/*fn sort_file (txt:Vec<String>)
{
    let mut categories :Vec<Category>= Vec::new();
    let mut value_multiprotoline:Value;
    let mut multiprotoline=false;
    for protoline in txt
    {
        let mut categorie=true;
        let mut categorie_word="";
        let mut attribute_word="";
        let mut value_word:Value;
        if multiprotoline
        {
            let mut protoline_iter = protoline.chars();
            while protoline_iter[0] != '_'
            {
                value_multiprotoline+=protoline;
            }
            multiprotoline=false;
        }



        for charactere in protoline.chars()
        {

            if charactere=='_'
            {
                let new_decoupe :Vec<&str>= txt.split_at(whitespace).collect(); //problemos
                let second_decoupe:Vec<&str>=new_decoupe[0].split(".").collect();
                if new_decoupe.len()==1
                {
                    multiprotoline=true;
                }
                else
                {
                    value_word=new_decoupe[1]; 
                    categorie_word=second_decoupe[0];
                    attribute_word=second_decoupe[1];
                    let attribute_word_final=attribute_word.to_string();
                    let categorie_word_final=categorie_word.to_string();
                    let attribut_found=Attribute{ key:attribute_word_final, value:value_word};

                    for nom in categories
                    {
                        if categorie_word==nom.name
                        {
                            let mut categorie=false;
                            nom.attributes.push(attribut_found);
                        }
                    }
                    if categorie
                    {
                        let attribut_of_cat:Vec<Attribute>=Vec::new();
                        attribut_of_cat.push(attribut_found);
                        let new_cat= Category{ name: categorie_word_final , attributes: attribut_of_cat };
                        categories.push(new_cat);
                    }
             
                }
            }
            
        }
    }
}
*/


fn main() {
    let file = read_file("star_reader_tests.mmcif");
    // println!("{}",file);
    parse_star(file)
}
