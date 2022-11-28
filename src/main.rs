use std::fs::File;
use std::io::Read;


#[derive(Debug, Clone)]
struct Category {
    name: String,
    attributes: Vec<Attribute>,
}
#[derive(Debug, Clone)]
struct Attribute {
    key: String,
    values: String,
}

fn read_file(filepath: &str) -> String {
    let mut file = File::open(filepath).expect("file not found");
    let mut content = String::new();
    file.read_to_string(&mut content).expect("cannot read");
    content
}

fn parse_star(txt: String) -> Vec<Category> {
    let chariot: Vec<&str> = txt.split(['\n']).collect();
    let mut txt: Vec<String> = Vec::new();

    for i in chariot {
        let mut protoline = String::from("");
        for j in i.chars() {
            if j != '#'
            //skip comments
            {
                let s = j.to_string();
                protoline += &s;
            } else {
                break;
            }
        }
        let line = protoline.clone();
        // println!("{}",line);
        let length = &line.chars().count();
        if length > &0
        //skip empty lines
        {
            txt.push(line);
        }
    }
    let output: Vec<Category> = sort_file(txt);
    output
}

fn sort_file(txt: Vec<String>) -> Vec<Category> {
    let mut categorie_word = String::from("");
    let mut attribute_word = String::from("");
    let mut attribute_vec:Vec<String>=Vec::new();
    let mut valeur_vec:Vec<String>=Vec::new();
    let mut valeur_vec_tot:Vec<Vec<String>>=Vec::new();
    let mut valeur = String::from("");
    let mut boucle=false;
    let mut multiline = false;
    let mut categories: Vec<Category> = Vec::new();
    for i in txt {
        if multiline == false {
            if i== "loop_" {
                boucle=true;
            }
            else{
                if boucle==true {
                    let mut compteur =0;
                    while i.chars().nth(0).unwrap().to_string() == "_" {
                        let espace: Vec<&str> = i.split_whitespace().collect();
                        let second_decoupe: Vec<&str> = espace[0].split(".").collect();
                        categorie_word = second_decoupe[0].to_string();
                        attribute_vec.push(second_decoupe[1].to_string());
                        compteur+=1;
                    }
                    for _ in 0..compteur{
                        let espace: Vec<&str> = i.split_whitespace().collect();
                        for k in espace{
                            valeur_vec.push(k.to_string());
                        }
                        let val=valeur_vec.clone();
                        valeur_vec_tot.push(val);
                        valeur_vec.clear();
                        
                    }
                    let results = rechercher_categorie(&categorie_word, &categories);
                    let nouvelle_valeur=valeur.clone();
                    match results {
                        Some(result) => {
                            
                            let attribute_found = category_exist(nouvelle_valeur, &attribute_word);
                            let indexes:usize=result.clone() as usize;
                            categories[indexes].attributes.push(attribute_found);
                        }
                        None => {
                            let new_cat = category_dont_exist(nouvelle_valeur, &categorie_word, &attribute_word);
                            categories.push(new_cat);
                        }
                    }

                }
                else{
                    if i.chars().nth(0).unwrap().to_string() == "_" {
                        
                        let espace: Vec<&str> = i.split_whitespace().collect();
                        let second_decoupe: Vec<&str> = espace[0].split(".").collect();
                        categorie_word = second_decoupe[0].to_string();
                        attribute_word = second_decoupe[1].to_string();
                        if espace.len() >= 2 {
                            not_multiline(&mut categories, espace, &categorie_word, &attribute_word);
                            
                        } else {
                            multiline = true;
                        }
                    }
                
                    else {
                        println!("{i}");
                        if i.chars().last().unwrap().to_string() != ";" {
                            valeur += &i;
                            
                        } 
                    

                        else {
                            let cat_word = categorie_word.clone();
                            let results = rechercher_categorie(&cat_word, &categories);
                            let nouvelle_valeur=valeur.clone();
                            match results {
                                Some(result) => {
                                    let attribute_found = category_exist(nouvelle_valeur, &attribute_word);
                                    
                                    let indexes:usize=result.clone() as usize;
                                    categories[indexes].attributes.push(attribute_found);  

                                    multiline=false;
                                }
                                None => {
                                    let new_cat = category_dont_exist(nouvelle_valeur, &categorie_word, &attribute_word);
                                    categories.push(new_cat);
                                    multiline=false;
                                }
                            }
                        }
                    }
                }
            }
        } 
    }
    return categories;
}

fn not_multiline( categories: &mut Vec<Category>, espace: Vec<&str>, categorie_word: &String, attribute_word: &String) {
    let mut valeur = String::from("");
    for i in espace {
        valeur = i.to_string();
    }
    let results = rechercher_categorie(&categorie_word, categories);
    let nouvelle_valeur=valeur.clone();
    match results {
        Some(result) => {
            let attribute_found = category_exist(nouvelle_valeur, &attribute_word);
            let indexes:usize=result.clone() as usize;
            categories[indexes].attributes.push(attribute_found);
        }
        None => {
            let new_cat = category_dont_exist(nouvelle_valeur, &categorie_word, &attribute_word);
            categories.push(new_cat);
        }
    }
}

fn rechercher_categorie<'a>( categorie: &'a String, veccategorie: &'a Vec<Category> ) -> Option< i32> {
    for i in veccategorie {
        let mut compteur=0;
        if categorie.to_string() == i.name {
            return Some(compteur);
        }
        compteur+=1
    }
    return None;
}



fn category_exist(value_final: String, attribute_word: &String) -> Attribute {
    let attribute_found = Attribute {
        key: (attribute_word).to_string(),
        values: (value_final).to_string(),
    };
    return attribute_found;
}
fn category_dont_exist(value_final: String, categorie_word: &String, attribute_word: &String) -> Category {
    let mut attribut_of_cat: Vec<Attribute> = Vec::new();
    let attribute_found = Attribute {
        key: (attribute_word).to_string(),
        values: value_final.to_string(),
    };
    attribut_of_cat.push(attribute_found);
    let new_cat = Category {
        name: (categorie_word).to_string(),
        attributes: attribut_of_cat,
    };
    return new_cat;
}

fn main() {
    let file = read_file("test_unit.mmcif");
    // println!("{}",file);
     
    parse_star(file);
}
