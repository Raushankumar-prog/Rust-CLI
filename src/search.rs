use std::fs;

pub fn search(query:&str,file:&str){
    let mut ve=Vec::new();
    let query=query;
    let file=file;
 
     let hg=fs::read_to_string(file).expect("this is error");
      
       

     for words in hg.split_whitespace(){
        if words.to_string() == query {
            ve.push(words);
        }
     }
  println!("{:?}",ve);
}