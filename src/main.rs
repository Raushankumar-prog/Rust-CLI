mod one;
mod search;
use std::env;
use one::Afd;
use search::search;

fn main() {
    let args:Vec<String>=env::args().collect();
    let config=Afd::getting_command(&args);
    let query=config.query;
    let file=config.file;

    let  ve= search(query,file);
          
   println!("{:?}",ve);
}



