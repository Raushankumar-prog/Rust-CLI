
pub struct Afd<'a>{
   pub query:&'a String,
    pub file:&'a String
}

 impl <'a> Afd<'a>{
    pub fn getting_command(args:&'a[String]) -> Afd<'a>{
      
        let query =&args[1] ;
        let file=&args[2];
    
        Afd{query,file}
       
    }
}
