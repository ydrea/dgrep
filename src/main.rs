use std::process;
use std::env;
use dgrep::Config;

fn main() {
    //args         type        iterator    to collection...
    //          ...of type Vector
        let args : Vec<String> = env::args().collect();
    
        //  invoke parse
        let config = Config::parse(&args).unwrap_or_else(|err| {
            println!("nece parsat: {}", err);
            process::exit(1)
        } );
        
        println!("Traži {}", config.query);
        println!("u fajlu {}", config.poem);
    
      if let Err (e) = dgrep::run(config) {
        println!("necu! {} ", e);
        process::exit(1)
        
      }
    
    }
    