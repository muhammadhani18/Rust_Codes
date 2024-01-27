use std::process::exit;
use std::collections::HashMap;

fn main() {
    run();
}

fn run() {

    let s: String = "{'id' : 'a12', 'name' : 'Hani', 'address' : 'asass'}".to_string();
    parse(s);
}

fn parse(mut s: String) {

    let mut buffer = String::new();

 
    
    let mut in_bracket = false;
    let mut out_bracket = false;

    //start parsing till found '}'
    if s.starts_with('{') && s.ends_with('}')
    {
        
        //remove starting and ending bracket
        s.remove(0);
        s.remove(s.len()-1);

        for c in s.chars() { 

            if c == '\''  && in_bracket == false 
            { 
             
                in_bracket = true; 
                out_bracket = false;
            
            }
            else if c == '\''  && in_bracket == true 
            {
             
                out_bracket = true;
                in_bracket = false;
                buffer.push(' ');
                                            
            }
            else if in_bracket && out_bracket == false
            { 
            
                buffer.push(c);
            
            }
        }

        //split the key and value by spaces
        let words: Vec<&str> = buffer.split_whitespace().collect();

        store_in_hasmap(words);
    }
    else {
        println!("Wrong JSON.");
        exit(-1);
    }
    

}

fn store_in_hasmap(words: Vec<&str>){

    let mut store_json: HashMap<String, String> = HashMap::new();

    // Iterate through the words in pairs (key, value)
    for i in (0..words.len()).step_by(2) 
    {
        // Check if there's a corresponding value for the key
        if i + 1 < words.len() 
        {
            store_json.insert(words[i].to_owned(), words[i + 1].to_owned());
        } 
        else 
        {
            // Handle potential error: odd number of words
            println!("Warning: Odd number of words, skipping the last word.");
        }
    }


    for (key, value) in &store_json {
        println!("{}: {}", key, value);
    }
}