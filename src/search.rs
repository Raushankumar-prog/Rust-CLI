use std::fs;
 
  #[allow(dead_code)]
  
pub fn search(query:&str,file:&str)->Result<Vec<String>,Box<dyn std::error::Error>>{
    let mut ve=Vec::new();
    let query=query;
    let file=file;
 
     let hg=fs::read_to_string(file).expect("this is error");
      
       

     for words in hg.lines(){
        let  words=words.to_lowercase();
        let query=&query.to_lowercase();
        if words.contains(query) {
            ve.push(words);
        }
     }
     let ve=ve;
  Ok(ve)
}