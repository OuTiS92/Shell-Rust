use std::{fmt::Error, io, path::PathBuf};
use std::path::Path;

fn main(){

    let vars : Vec<String> = std::env::args().collect(); 
    loop {
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        let line = line.trim();
        println!( "{}" , line);
    }
}


fn run_process(vars: &Vec<String> , command :&str ) -> Result< () , () > {
    Ok(())
}


fn find_binary(commnad: &str) -> Result< PathBuf ,  std::io::Error> {
    fn search (command :&str , path:&Path )-> Result<() , std::io::Error> {
        for entry in std::fs::read_dir(path)? {

        }
        Err(std::io::ErrorKind::NotFound.into())
    };
    todo!()
}
 