

pub fn getting_command(args:&[String]) -> (&str,&str){
   
    let query =&args[1] ;
    let file=&args[2];

    (query,file)
   
}