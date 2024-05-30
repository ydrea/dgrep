use std::error::Error;
use std::fs;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let pjesma = fs::read_to_string(config.poem)?;

    for i in search(&config.query, &pjesma){

    println!("Pevaj:\n{}", i);}
    Ok(())
    
}

pub struct Config {
    pub query: String,
    pub poem: String
}

impl Config {

    pub fn parse (args: &[String]) -> Result<Config, &str> {

        if args.len() < 3 {
            return Err("fali argsa");
        }
        // let wtf = &args[0];
        let query = args[1].clone();
        let poem = args[2].clone();
    // println!("{}",wtf);
        
   Ok( Config {query, poem})
    }
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn search<'a>(query: &'a str, tring: &'a str) -> Vec<&'a str> {
    let mut rez = Vec::new();

    for i in tring.lines(){
    if i.contains(query){
    rez.push(i);
}
}rez
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pass() { 
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn onesearchresult() {
        let query = "om";
        let tring = "\
        alom
        edobok
        zeko";
assert_eq!(vec!["alom"], search(query, tring));
        println!("Fail!");
    }
}
