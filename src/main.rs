mod one;
mod search;
use std::env;
use one::getting_command;
use search::search;

fn main() {
    let args:Vec<String>=env::args().collect();
    let (query,file)=getting_command(&args);
    let  ve= search(query,file);
          
   println!("{:?}",ve);
}



