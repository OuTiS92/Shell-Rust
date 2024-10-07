use std::collections::HashMap;
use std::{fmt::Error, io, path::PathBuf};
use std::path::{self, Path};

use libc::ENETUNREACH;

fn main(){

    let raw_vars : Vec<String> = std::env::args().collect(); 
    let var: HashMap<String,String> = raw_vars.into_iter().map(|v| {
        let mut parts = v.split("=");
        let key = parts.next().unwrap();
        let val=parts.next().map(|v| v.to_owned()).unwrap_or(String::new());
        (key.to_owned(),val)
    }).collect();
    
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


fn find_binary(commnad: &str , path: &str) -> Result< PathBuf ,  std::io::Error> {
    fn search (command :&str , path:&Path )-> Result<() , std::io::Error> {
        for entry in std::fs::read_dir(path)? {
            if let Ok(entry) = entry{
                if let Ok(met) = entry.metadata(){
                    if met.is_file() || met.is_symlink() {
                        if let Some(name) = entry.path().file_name(){
                            if name == command{
                                if met.is_symlink(){
                                    panic!("Running symlinks not supported");
                                }
                                return  Ok(());
                            }
                        }
                    }
                }
            }

        }
        Err(std::io::ErrorKind::NotFound.into())
    }
    if let Ok(mut dir) = std::env::current_dir(){
        if let Ok(()) = search(commnad, &dir){
            dir.push(commnad);
            return  Ok(dir);
        }
    }
    for entry in path.split(":"){
        let mut path = PathBuf::from(entry);
        if let Ok(()) = search(commnad, &path){
            path.push(commnad);
            return  Ok(path);
        }
    }
    Err(std::io::ErrorKind::NotFound.into())
}
 