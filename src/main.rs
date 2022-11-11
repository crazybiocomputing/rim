use std::fs::File;
use std::io::Read;
struct Category {
    name: String,
    attributes: Vec<Attribute>
  }
  struct Attribute {
    key: String,
    values: String
  }  

fn read_file(filepath:&str) -> String{
    let mut file = File::open(filepath).expect("file not found");
    let mut content = String::new();
    file.read_to_string(&mut content).expect("cannot read");
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

fn sort_file(txt:Vec<String>){
    let mut categorie_word:String;
    let mut attribute_word:String;
    let mut multiline=false;
    let mut categories:Vec<Category>=Vec::new();
    for i in txt{
        if multiline == false {
            if i.chars().nth(0).unwrap().to_string()=="_"
            {
                let espace :Vec<&str>= i.split_whitespace().collect();
                let second_decoupe:Vec<&str>=espace[0].split(".").collect();
                categorie_word=second_decoupe[0].to_string();
                attribute_word=second_decoupe[1].to_string();
                if espace.len()>=2{ 
                    not_multiline(&categories,espace,categorie_word,attribute_word);
                }
                else{
                    multiline=true;
                }
            }
        }
        else{
            let mut valeur:String;
            if i.chars().last().unwrap().to_string()!=";"{
                valeur+= &i;
            }
            else{
                let mut results =rechercher_categorie(categorie_word,&categories);
                if results.0{
                    let mut attribute_found=category_exist(valeur, attribute_word);
                    results.1.attributes.push(attribute_found);
                }
                    
                else{
                    let mut new_cat=category_dont_exist(valeur, categorie_word, attribute_word);
                    categories.push(new_cat);
                }
                
            }
        }
    }
}


fn not_multiline(categories:&Vec<Category>,espace:Vec<&str>,categorie_word:String,attribute_word:String){
    let mut valeur:String;
    for i in espace{
        valeur= i.to_string();
        
    }
    let mut results =rechercher_categorie(categorie_word,categories);
        if results.0{
            let mut attribute_found=category_exist(valeur,attribute_word);
            results.1.attributes.push(attribute_found);
        }
            
        else{
            let mut new_cat=category_dont_exist(valeur, categorie_word, attribute_word);
            categories.push(new_cat);
        }
    }


fn rechercher_categorie(categorie:String, veccategorie:&Vec<Category>)->(bool,&Category){
    let mut result=true;
    let mut cat:Category; 
    for i in veccategorie{
        if categorie==i.name{
            result=true;
            cat=i.to_vec();
        }
        else{
            result=false
        }
    }
    return (result,&cat)
}

    fn category_exist(value_final:String,attribute_word:String)-> Attribute{

        let mut attribute_found=Attribute{ key:attribute_word, values:value_final};
        return attribute_found;
    }
    fn category_dont_exist(value_final:String,categorie_word:String,attribute_word:String)-> Category{
    
        let attribut_of_cat:Vec<Attribute>=Vec::new();
        let mut attribute_found=Attribute{ key:attribute_word, values:value_final};
        attribut_of_cat.push(attribute_found);
        let new_cat= Category{ name: categorie_word , attributes: attribut_of_cat };
        return new_cat;
    }

    fn main() {
        let file = read_file("star_reader_tests.mmcif");
        // println!("{}",file);
        parse_star(file)
    }