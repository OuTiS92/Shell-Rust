use std::collections::HashMap;
use std::{fmt::Error, io, path::PathBuf};
use std::path::{self, Path};

use libc::ENETUNREACH;

fn main(){

    let vars  = std::env::vars().into_iter().collect(); 
    dbg!(&vars);

    loop {
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        let line = line.trim();
        run_process(&vars, &line).unwrap();
    }
}

fn run_process(vars: &HashMap<String,String> , commnad: &str) -> Result< () , () > {
    let bin = find_binary(commnad, &vars["PATH"]);
    println!("{:?}" , bin);
    panic!();
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
 